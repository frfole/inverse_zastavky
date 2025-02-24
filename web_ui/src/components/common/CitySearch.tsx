import "./CitySearch.css"
import {useMap} from "react-leaflet";
import {ChangeEvent, useContext, useEffect, useState} from "react";
import {BaseCity} from "../../model/model.ts";
import {searchBaseCity} from "../../data/interact.ts";
import {AppContext} from "../../data/app-context.ts";
import {approx_distance} from "../../utils/geo.ts";

export function CitySearch() {
    const map = useMap()
    const [cities, setCities] = useState<BaseCity[]>([])
    const appState = useContext(AppContext)

    async function handleSearchChange(e: ChangeEvent<HTMLInputElement>) {
        const query = e.target.value;
        if (query.length == 0) {
            setCities([])
            return
        }
        setCities(await searchBaseCity(query + "%"))
    }

    useEffect(() => {
        let cancelFence = false
        const query = appState.citySearch
        if (query.length == 0) {
            setCities([])
        } else {
            searchBaseCity(query + "%")
                .then(cities => {
                    if (cancelFence) return
                    setCities(cities.slice(0, 20))
                })
        }
        return () => {
            cancelFence = true
        }
    }, [appState.citySearch])

    return (
        <div className="CitySearch__wrapper">
            <input type="text" onChange={handleSearchChange}/>
            {cities
                .map(city => [city, approx_distance([city.lat, city.lon], [map.getCenter().lat, map.getCenter().lng])])
                .sort(([, aDist], [, bDist]) => aDist as number - (bDist as number))
                .map(pair => {
                    const city = pair[0] as BaseCity
                    const dist = pair[1] as number
                    return (
                        <button
                            key={city.lat + city.lon + city.name}
                            className="CitySearch__button"
                            onClick={() => {
                                map.flyTo([city.lat, city.lon], 13, {
                                    duration: 2
                                })
                            }}>
                            {city.name + " (" + dist.toFixed(1) + " km)"}
                        </button>
                    )
                })}
        </div>
    )
}