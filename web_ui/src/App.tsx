import {useCallback, useEffect, useReducer, useState} from "react"
import "./App.css"
import {MapContainer, TileLayer} from "react-leaflet";
import {BBox, Mode} from "./model/model.ts";
import {MapRef} from "react-leaflet/MapContainer";
import {createStation, getCityRemap, getStations, getStats} from "./data/interact.ts";
import {StationMarker} from "./components/browse/StationMarker.tsx";
import {EditableStationMarker} from "./components/editor/EditableStationMarker.tsx";
import {AddStationDialog} from "./components/editor/AddStationDialog.tsx";
import {LeafletMouseEvent} from "leaflet";
import {ActionType, appReducer} from "./data/app-reducer.ts";
import ReactModal from "react-modal";
import {AppContext, AppDispatchContext} from "./data/app-context.ts";
import {LocateView} from "./components/locate/LocateView.tsx";
import {CitySearch} from "./components/common/CitySearch.tsx";
import {StationSearch} from "./components/common/StationSearch.tsx";
import ModeSwitcher from "./components/common/ModeSwitcher.tsx";

ReactModal.setAppElement("#root")

function App() {
    const [map, setMap] = useState<MapRef>(null)
    const [state, dispatch] = useReducer(appReducer, {
        mode: Mode.Browse,
        stations: [],
        lastMapClick: null,
        stationSearch: "",
        citySearch: "",
        locateOffset: 0,
        cityRemap: {},
    })
    const [addStationDialog, setAddStationDialog] = useState(false)

    async function handleStationCreate(name: string) {
        if (!state.lastMapClick) return
        const station = await createStation(state.lastMapClick.lat, state.lastMapClick.lng, name);
        dispatch({
            type: ActionType.UpdateStation,
            station: station
        })
    }

    const handleMapMoveEnd = useCallback(async function handleMapMoveEnd() {
        if (map == null) return;
        const stations = await getStations(BBox.fromBounds(map?.getBounds()))
        dispatch({
            type: ActionType.SetStations,
            new_stations: stations,
        })
    }, [map]);

    const handleMapClick = useCallback(function handleMapClick(event: LeafletMouseEvent) {
        if (map == null) return;
        if (event.target._container !== event.originalEvent.target) return
        dispatch({
            type: ActionType.MapClick,
            latLon: event.latlng,
        })
        switch (state.mode) {
            case Mode.Editor:
                setAddStationDialog(true)
                break
        }
    }, [map, state.mode])

    useEffect(() => {
        let cancelFence = false;
        getCityRemap().then(remap => {
            if (cancelFence) return;
            dispatch({
                type: ActionType.SetCityRemap,
                remap: remap
            })
        })
        return () => {
            cancelFence = true
        }
    }, []);

    useEffect(() => {
        if (map == null) {
            return () => {
            }
        }
        let cancelFence = false
        getStations(BBox.fromBounds(map?.getBounds()))
            .then(stations => {
                if (cancelFence) return;
                dispatch({
                    type: ActionType.SetStations,
                    new_stations: stations,
                })
            })
        return () => {
            cancelFence = true
        }
    }, [map])

    useEffect(() => {
        if (map == null) return () => {
        }
        setAddStationDialog(false)
        map.on("moveend", handleMapMoveEnd)
        map.on("click", handleMapClick)
        return () => {
            map.off("moveend", handleMapMoveEnd)
            map.off("click", handleMapClick)
        }
    }, [handleMapClick, handleMapMoveEnd, map])

    return (
        <AppContext.Provider value={state}>
            <AppDispatchContext.Provider value={dispatch}>
                <MapContainer
                    center={[49.7, 15.8]}
                    zoom={8}
                    ref={setMap}>
                    <TileLayer
                        attribution="&copy; <a href='https://www.openstreetmap.org/copyright'>OpenStreetMap</a> contributors"
                        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"/>
                    <div className="map-overlay wrapper-mode-switcher">
                        <ModeSwitcher/>
                    </div>
                    <AddStationDialog open={addStationDialog} onClose={() => setAddStationDialog(false)}
                                      onAdd={handleStationCreate}/>
                    {state.mode == Mode.Locate && <LocateView/>}
                    <div className="map-overlay App__CitySearch">
                        <CitySearch/>
                    </div>
                    <div className="map-overlay App__StationSearch">
                        <StationSearch/>
                    </div>
                    <>
                        {state.stations.map(station => {
                            switch (state.mode) {
                                case Mode.Editor:
                                    return <EditableStationMarker key={station.stop_id} station={station}/>
                                case Mode.Browse:
                                    return <StationMarker key={station.stop_id} station={station} popup/>
                            }
                        })}
                    </>
                    <div className="map-overlay App_GetStats">
                        <button onClick={async () => {
                            const stats = await getStats()
                            alert(JSON.stringify(stats, null, 2))
                        }}>Get stats
                        </button>
                    </div>
                </MapContainer>
            </AppDispatchContext.Provider>
        </AppContext.Provider>
    )
}

export default App
