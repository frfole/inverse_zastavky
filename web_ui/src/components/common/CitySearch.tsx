import "./CitySearch.css"
import {useMap} from "react-leaflet";
import {ChangeEvent, useContext, useEffect, useState} from "react";
import {BaseCity} from "../../model/model.ts";
import {searchBaseCity} from "../../data/interact.ts";
import {AppContext} from "../../data/app-context.ts";

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
            {cities.map(city => (
                <button
                    key={city.lat + city.lon + city.name}
                    className="CitySearch__button"
                    onClick={() => {
                        map.flyTo([city.lat, city.lon], 13, {
                            duration: 2
                        })
                    }}>
                    {city.name}
                </button>
            ))}
        </div>
    )
}