import "./StationMarker.css"
import {Station} from "../model/model.ts";
import {Marker, Popup, Tooltip} from "react-leaflet";

interface StationMarkerProps {
    station: Station
}

export function StationMarker({station}: StationMarkerProps) {
    return (
        <Marker key={station.stop_id} position={[station.lat, station.lon]}>
            <Popup>
                <ul className="name-list">
                    {station.names.map(name => (<li key={name} className="name-entry">{name}</li>))}
                </ul>
            </Popup>
            <Tooltip>
                <ul className="name-list">
                    {station.names.map(name => (<li key={name} className="name-entry">{name}</li>))}
                </ul>
            </Tooltip>
        </Marker>
    )
}