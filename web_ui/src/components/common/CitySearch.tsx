import "./CitySearch.css"
import {useMap} from "react-leaflet";
import {ChangeEvent, useState} from "react";
import {BaseCity} from "../../model/model.ts";
import {searchBaseCity} from "../../data/interact.ts";

export function CitySearch() {
    const map = useMap()
    const [cities, setCities] = useState<BaseCity[]>([])

    async function handleSearchChange(e: ChangeEvent<HTMLInputElement>) {
        const query = e.target.value;
        if (query.length == 0) {
            setCities([])
            return
        }
        setCities(await searchBaseCity(query + "%"))
    }

    return (
        <div className="CitySearch__wrapper">
            <input type="text" onChange={handleSearchChange}/>
            {cities.map(city => (
                <>
                    <br key={city.lat + city.lon + 1}/>
                    <button key={city.lat + city.lon}
                            className="CitySearch__button"
                            onClick={() => {
                                map.flyTo([city.lat, city.lon], 13, {
                                    duration: 2
                                })
                            }}>
                        {city.name}
                    </button>
                </>
            ))}
        </div>
    )
}