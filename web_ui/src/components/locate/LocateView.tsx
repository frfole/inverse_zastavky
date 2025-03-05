import "./LocateView.css"
import {FormEvent, useCallback, useContext, useEffect, useReducer, useRef} from "react";
import {AppContext, AppDispatchContext} from "../../data/app-context.ts";
import {getBaseStations, getChainStations, locateById, locateByLoc} from "../../data/interact.ts";
import {LocateActionType, locateReducer} from "../../data/locate-reducer.ts";
import {Circle, Tooltip, useMap, useMapEvent} from "react-leaflet";
import {StationMarker} from "../browse/StationMarker.tsx";
import {ActionType} from "../../data/app-reducer.ts";
import {BaseStation, BBox, Station} from "../../model/model.ts";
import {LocateState} from "../../data/locate-state.ts";

function stationColor(target: Station, state: LocateState): string | undefined {
    if (state.chainStations[state.selectedIdx] && target.names.includes(state.chainStations[state.selectedIdx].name)) {
        return "#ff4d00"
    } else if (state.chainStations[state.selectedIdx + 1] && target.names.includes(state.chainStations[state.selectedIdx + 1].name)) {
        return "#ffdd00"
    }
}

function baseStationColor(target: BaseStation, state: LocateState): string | undefined {
    if (state.chainStations[state.selectedIdx] && target.name.indexOf(state.chainStations[state.selectedIdx].name) >= 0) {
        return "#ff4d00"
    } else if (state.chainStations[state.selectedIdx + 1] && target.name.indexOf(state.chainStations[state.selectedIdx + 1].name) >= 0) {
        return "#ffdd00"
    } else {
        return "#008cff"
    }
}

export function LocateView() {
    const appState = useContext(AppContext)
    const appDispatch = useContext(AppDispatchContext)
    const [state, dispatch] = useReducer(locateReducer, {
        offset: appState.locateOffset,
        limit: 20,
        selectedIdx: 0,
        chainStations: [],
        baseStations: [],
    })
    const inputOffset = useRef<HTMLInputElement>(null)

    useEffect(() => {
        dispatch({
            type: LocateActionType.SetOffset,
            offset: appState.locateOffset,
        })
    }, []);

    useEffect(() => {
        let cancelFence = false
        getChainStations(state.offset, state.limit)
            .then(chainStations => {
                if (cancelFence) return;
                dispatch({
                    type: LocateActionType.SetChainStations,
                    newStations: chainStations
                })
                appDispatch({
                    type: ActionType.SearchStations,
                    query: chainStations[state.selectedIdx].name
                })
                appDispatch({
                    type: ActionType.SearchCity,
                    query: chainStations[state.selectedIdx].name.split(",", 1)[0].split(" [")[0]
                })
            })
        if (inputOffset.current) inputOffset.current.value = String(state.offset + state.selectedIdx)
        appDispatch({
            type: ActionType.SetLocateOffset,
            newOffset: state.offset + state.selectedIdx
        })
        return () => {
            cancelFence = true
        }
    }, [state.offset, state.limit, state.selectedIdx, appDispatch]);

    const handleListingChange = useCallback((event: FormEvent<HTMLFormElement>) => {
        event.preventDefault()
        const formData = new FormData(event.currentTarget);
        const formJson = Object.fromEntries((formData).entries());
        dispatch({
            type: LocateActionType.SetOffset,
            offset: Number.parseInt(formJson.offset.toString())
        })
    }, [])

    function handleSelectRow(idx: number) {
        dispatch({
            type: LocateActionType.SetSelectedIdx,
            idx: idx
        })
    }

    function handleAdvance() {
        dispatch({
            type: LocateActionType.Advance
        })
    }

    useMapEvent("click", async (e) => {
        if (e.target._container !== e.originalEvent.target) {
            return
        }
        const station = await locateByLoc(state.chainStations[state.selectedIdx], e.latlng.lat, e.latlng.lng);
        appDispatch({
            type: ActionType.UpdateStation,
            station: station
        })
        dispatch({
            type: LocateActionType.Advance
        })
    })

    const map = useMap()

    useMapEvent("moveend", async () => {
        const newStations = await getBaseStations(BBox.fromBounds(map.getBounds()))
        dispatch({
            type: LocateActionType.SetBaseStations,
            newStations: newStations
        })
    })

    return (
        <>
            <div className="ChainContainer__wrapper">
                <table>
                    <thead>
                    <tr>
                        <th>o</th>
                        <th>chain hash</th>
                        <th>pos</th>
                        <th>station name</th>
                    </tr>
                    </thead>
                    <tbody>
                    {state.chainStations.map((chainStation, idx) => (
                        <tr className={
                            (state.selectedIdx == idx ? "ChainContainer__row-selected " : "") +
                            (chainStation.stop_id == null ? "ChainContainer__row-undone " : "ChainContainer__row-done ") +
                            "ChainContainer__row"
                        }
                            onClick={() => handleSelectRow(idx)}
                            key={chainStation.chain_hash + chainStation.pos}>
                            <td>{state.offset + idx}</td>
                            <td>{chainStation.chain_hash}</td>
                            <td>{chainStation.pos}</td>
                            <td>{chainStation.name}</td>
                        </tr>
                    ))}
                    </tbody>
                </table>
                <form onSubmit={handleListingChange}>
                    <input type="number" name="offset" ref={inputOffset} placeholder="offset"/>
                    <input type="submit" value="Go"/>
                </form>
                <button onClick={handleAdvance}>next</button>
            </div>
            <>
                {map.getZoom() > 14 && state.baseStations.map(base => {
                    return <Circle
                        key={base.lat * base.lon + base.name} center={[base.lat, base.lon]} radius={5}
                        interactive={false}
                        color={baseStationColor(base, state)}>
                        <Tooltip permanent={true} opacity={0.5}>{base.name}</Tooltip>
                    </Circle>
                })}
            </>
            <>
                {map.getZoom() > 11 && appState.stations.map(station => {
                    return <StationMarker
                        key={station.stop_id}
                        station={station}
                        color={stationColor(station, state)}
                        onClick={async () => {
                            const new_station = await locateById(state.chainStations[state.selectedIdx], station);
                            appDispatch({
                                type: ActionType.UpdateStation,
                                station: new_station
                            })
                            dispatch({
                                type: LocateActionType.Advance
                            })
                        }}/>
                })}
            </>
        </>
    )
}