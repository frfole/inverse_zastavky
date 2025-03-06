import "./LocateView.css"
import {FormEvent, useCallback, useContext, useEffect, useReducer, useRef} from "react";
import {AppContext, AppDispatchContext} from "../../data/app-context.ts";
import {
    getBaseStations,
    getChainStations,
    getChainStationsByHash,
    locateById,
    locateByLoc, suggestChainCities,
    suggestChainStations
} from "../../data/interact.ts";
import {LocateActionType, locateReducer} from "../../data/locate-reducer.ts";
import {Circle, Polyline, Tooltip, useMap, useMapEvent} from "react-leaflet";
import {StationMarker} from "../browse/StationMarker.tsx";
import {ActionType} from "../../data/app-reducer.ts";
import {BaseStation, BBox, ChainStationsSuggestion, Station} from "../../model/model.ts";
import {LocateState} from "../../data/locate-state.ts";
import {LatLngBounds, LatLngExpression} from "leaflet";

function stationColor(target: Station, state: LocateState): string | undefined {
    if (state.chainStations[state.selectedIdx] && target.names.includes(state.chainStations[state.selectedIdx].name)) {
        return "#ff4d00"
    } else if (state.chainStations[state.selectedIdx + 1] && target.names.includes(state.chainStations[state.selectedIdx + 1].name)) {
        return "#ffdd00"
    } else if (state.selectedIdx - 1 >= 0 && state.chainStations[state.selectedIdx - 1] && target.names.includes(state.chainStations[state.selectedIdx - 1].name)) {
        return "#00e1ff"
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
        stationsSuggestions: [],
        citiesSuggestions: [],
        reload: false,
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
                let cityQuery = chainStations[state.selectedIdx].name.split(",", 1)[0].split(" [")[0];
                if (cityQuery in appState.cityRemap) {
                    console.log("City")
                    cityQuery = appState.cityRemap[cityQuery]
                }
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
                    query: cityQuery
                })
                suggestChainStations(chainStations[state.selectedIdx].chain_hash)
                    .then(suggestion => {
                        dispatch({
                            type: LocateActionType.SetStationsSuggestions,
                            newSuggestions: suggestion,
                        })
                    })
                suggestChainCities(chainStations[state.selectedIdx].chain_hash)
                    .then(suggestion => {
                        dispatch({
                            type: LocateActionType.SetCitiesSuggestions,
                            newSuggestions: suggestion,
                        })
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
    }, [state.offset, state.limit, state.selectedIdx, appDispatch, state.reload, appState.cityRemap]);

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
            type: LocateActionType.SetOffset,
            offset: state.offset + idx
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

    async function acceptStationsSuggestion(suggestion: ChainStationsSuggestion) {
        const stops = suggestion.path.map(a => a == null ? null : a[2])
        const chainStations = await getChainStationsByHash(state.chainStations[state.selectedIdx].chain_hash)
        for (let i = 0; i < stops.length; i++) {
            const stop = stops[i];
            if (stop == null) continue
            console.log(chainStations[i], stop)
            await locateById(chainStations[i], stop)
        }
        dispatch({
            type: LocateActionType.Reload,
        })
    }

    return (
        <>
            <div className="ChainContainer__wrapper map-overlay">
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
                <button onClick={() => dispatch({
                    type: LocateActionType.SetOffset,
                    offset: state.offset + state.selectedIdx - 10
                })}>prev 10
                </button>
                <button onClick={handleAdvance}>next</button>
                <button onClick={() => dispatch({
                    type: LocateActionType.SetOffset,
                    offset: state.offset + state.selectedIdx + 10
                })}>next 10
                </button>
                <button onClick={() => dispatch({
                    type: LocateActionType.SetOffset,
                    offset: state.offset + state.selectedIdx + 20
                })}>next 20
                </button>
            </div>
            <div className="StationsSuggestion__wrapper map-overlay">
                {state.stationsSuggestions.sort((left, right) => left.len - right.len)
                    .slice(0, 20)
                    .map((suggestion, idx) => (
                        <button key={suggestion.len + " " + idx}
                                onPointerLeave={() => {
                                    dispatch({
                                        type: LocateActionType.SetSuggestionPreview,
                                        path: undefined
                                    })
                                }}
                                onPointerEnter={(event) => {
                                    const path = suggestion.path.filter(a => a != null)
                                        .map(a => [a[0], a[1]]) as LatLngExpression[]
                                    if (event.ctrlKey) {
                                        map.fitBounds(new LatLngBounds(path))
                                    }
                                    dispatch({
                                        type: LocateActionType.SetSuggestionPreview,
                                        path: path
                                    })
                                }}
                                onClick={async () => acceptStationsSuggestion(suggestion)}>
                            {suggestion.len.toFixed(2) + " km (" + suggestion.path.filter(a => a != null).length + "/" + suggestion.path.length + ")"}
                        </button>
                    ))}
            </div>

            <div className="CitiesSuggestion__wrapper map-overlay">
                {state.citiesSuggestions.sort((left, right) => left.len - right.len)
                    .slice(0, 20)
                    .map((suggestion, idx) => (
                        <button key={suggestion.len + " " + idx}
                                onPointerLeave={() => {
                                    dispatch({
                                        type: LocateActionType.SetSuggestionPreview,
                                        path: undefined
                                    })
                                }}
                                onPointerEnter={() => {
                                    dispatch({
                                        type: LocateActionType.SetSuggestionPreview,
                                        path: suggestion.path.filter(a => a != null) as LatLngExpression[]
                                    })
                                }}
                                onClick={() => {
                                    map.fitBounds(new LatLngBounds(suggestion.path as LatLngExpression[]))
                                }}
                        >
                            {suggestion.len.toFixed(2) + " km"}
                        </button>
                    ))}
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
                            const new_station = await locateById(state.chainStations[state.selectedIdx], station.stop_id);
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
            <>
                {state.suggestionPreview && <Polyline color="red" positions={state.suggestionPreview}></Polyline>}
            </>
        </>
    )
}