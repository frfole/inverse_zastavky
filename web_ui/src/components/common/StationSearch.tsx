import "./StationSearch.css"
import {useMap} from "react-leaflet";
import {ChangeEvent, useContext, useEffect, useState} from "react";
import {Station} from "../../model/model.ts";
import {searchStations} from "../../data/interact.ts";
import {AppContext} from "../../data/app-context.ts";
import {approx_distance} from "../../utils/geo.ts";

export function StationSearch() {
    const map = useMap()
    const [stations, setStations] = useState<Station[]>([])
    const appState = useContext(AppContext)

    async function handleSearchChange(e: ChangeEvent<HTMLInputElement>) {
        const query = e.target.value;
        if (query.length == 0) {
            setStations([])
            return
        }
        setStations((await searchStations("%" + query + "%")).slice(0, 10))
    }

    useEffect(() => {
        let cancelFence = false
        const query = appState.stationSearch
        if (query.length == 0) {
            setStations([])
        } else {
            searchStations("%" + query + "%")
                .then(stations => {
                    if (cancelFence) return
                    setStations(stations.slice(0, 10))
                })
        }
        return () => {
            cancelFence = true
        }
    }, [appState.stationSearch])

    return (
        <div className="StationSearch__wrapper">
            <input type="text" onChange={handleSearchChange}/>
            {stations
                .map(station => [station, approx_distance([station.lat, station.lon], [map.getCenter().lat, map.getCenter().lng])])
                .sort(([, aDist], [, bDist]) => aDist as number - (bDist as number))
                .map((pair) => {
                    const station = pair[0] as Station
                    const dist = pair[1] as number
                    return (
                        <button key={station.lat + station.lon}
                                className="StationSearch__button"
                                onClick={() => {
                                    map.flyTo([station.lat, station.lon], 15, {
                                        duration: 1
                                    })
                                }}>
                            {station.names[0] + " (" + dist.toFixed(1) + " km)"}
                        </button>
                    )
                })}
        </div>
    )
}