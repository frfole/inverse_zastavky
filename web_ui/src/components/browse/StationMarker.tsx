import "./StationMarker.css"
import {Station} from "../../model/model.ts";
import {Marker, Popup, Tooltip} from "react-leaflet";
import {useMemo} from "react";

interface StationMarkerProps {
    station: Station,
    popup?: boolean,
    onClick?: (stop_id: number) => void,
}

export function StationMarker(props: StationMarkerProps) {
    const eventHandlers = useMemo(
        () => ({
            click() {
                if (props.onClick) props.onClick(props.station.stop_id)
            },
        }),
        [props],
    )

    return (
        <Marker key={props.station.stop_id}
                position={[props.station.lat, props.station.lon]}
                eventHandlers={eventHandlers}>
            {props.popup &&
            <Popup>
                <ul className="name-list">
                    {props.station.names.map(name => (<li key={name} className="name-entry">{name}</li>))}
                </ul>
            </Popup>}
            <Tooltip>
                <ul className="name-list">
                    {props.station.names.map(name => (<li key={name} className="name-entry">{name}</li>))}
                </ul>
            </Tooltip>
        </Marker>
    )
}