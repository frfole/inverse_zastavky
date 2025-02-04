import "./StationMarker.css"
import "./EditableStationMarker.css"
import {Station} from "../model/model.ts";
import {Marker, Popup, Tooltip} from "react-leaflet";
import {useCallback, useContext, useMemo, useRef} from "react";
import {AppDispatchContext} from "../data/app-context.ts";
import {addStationName, moveStation, rmStation, rmStationName} from "../data/interact.ts";
import {ActionType} from "../data/app-reducer.ts";

interface EditableStationMarkerProps {
    station: Station
}

export function EditableStationMarker({station}: EditableStationMarkerProps) {
    const appDispatch = useContext(AppDispatchContext)
    const markerRef = useRef<L.Marker>(null)

    const eventHandlers = useMemo(
        () => ({
            async dragend() {
                const marker = markerRef.current
                const stopId = station.stop_id
                if (marker != null) {
                    const station = await moveStation(stopId, marker.getLatLng().lat, marker.getLatLng().lng)
                    appDispatch({
                        type: ActionType.UpdateStation,
                        station: station
                    })
                }
            },
        }),
        [appDispatch, station.stop_id],
    )

    const handleSubmitName = useCallback((async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault()
        const formData = new FormData(event.currentTarget);
        const formJson = Object.fromEntries((formData).entries());
        const name = formJson.name;
        const updated_station = await addStationName(station.stop_id, name as string);
        appDispatch({
            type: ActionType.UpdateStation,
            station: updated_station
        })
    }), [appDispatch, station.stop_id])

    const handleRemoveName = useCallback((async (name: string) => {
        const updated_station = await rmStationName(station.stop_id, name);
        appDispatch({
            type: ActionType.UpdateStation,
            station: updated_station
        })
    }), [appDispatch, station.stop_id])

    const handleRemoveStation = useCallback(async () => {
        await rmStation(station.stop_id)
        appDispatch({
            type: ActionType.RemoveStation,
            stop_id: station.stop_id
        })
    }, [appDispatch, station.stop_id])

    return (
        <Marker draggable={true} key={station.stop_id} position={[station.lat, station.lon]} eventHandlers={eventHandlers} ref={markerRef}>
            <Popup>
                <h3>Names</h3>
                <ul className="name-list">
                    {station.names.map(name => (
                        <li key={name} className="name-entry">
                            <button className="button-rm-name" onClick={async () => {
                                await handleRemoveName(name)
                            }}>R</button>
                            {name}
                        </li>
                    ))}
                </ul>
                <form onSubmit={handleSubmitName}>
                    <input type="text" name="name" required placeholder="additional name"/>
                    <input type="submit" value="Add name"/>
                </form>
                <button onClick={handleRemoveStation}>Remove station</button>
            </Popup>
            <Tooltip>
                <ul className="name-list">
                    {station.names.map(name => (
                        <li key={name} className="name-entry">{name}</li>
                    ))}
                </ul>
            </Tooltip>
        </Marker>
    )
}