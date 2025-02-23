import "./StationMarker.css"
import {Station} from "../../model/model.ts";
import {Marker, Popup, Tooltip} from "react-leaflet";
import {useMemo} from "react";
import L from "leaflet";

function markerSvg(color?: string) {
    if (!color) color = "#039de1"
    return `
<svg width="36" height="52" style="pointer-events: none">
    <path
        style="fill:${color};stroke-width:1.5;stroke: gray;pointer-events: visiblePainted;"
        d="m 2,2 m 16,0 m 0,0 c -8,0 -16,4 -16,16 0,8 16,32 16,32 0,0 16,-24 16,-32 0,-12 -8,-16 -16,-16 z" />
    <path
        style="fill: white; stroke: #9a9a9a; stroke-width: 2;pointer-events: visiblePainted;"
        d="m 2,2 m 16,10 c 6,0 6,6 6,6 0,6 -6,6 -6,6 -6,0 -6,-6 -6,-6 0,-6 6,-6 6,-6 z" />
</svg>`
}

interface StationMarkerProps {
    station: Station,
    popup?: boolean,
    onClick?: (stop_id: number) => void,
    color?: string,
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
                icon={L.divIcon({
                    html: markerSvg(props.color),
                    iconSize: [32, 48],
                    iconAnchor: [17, 50],
                    className: "",
                    tooltipAnchor: [17, -34],
                    popupAnchor: [0, -50]
                })}
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