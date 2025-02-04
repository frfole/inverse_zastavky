import {BBox, Station} from "../model/model.ts";
import {config} from "../config.ts";

export async function getStations(bbox: BBox): Promise<Station[]> {
    const url = new URL(config.api_endpoint + "/el_stations_bbox");
    url.searchParams.append("lat_from", String(bbox.lat_from))
    url.searchParams.append("lat_to", String(bbox.lat_to))
    url.searchParams.append("lon_from", String(bbox.lon_from))
    url.searchParams.append("lon_to", String(bbox.lon_to))
    return await fetch(url)
        .then(response => response.ok ? response.json() : Promise.reject())
        .then(data => data as Station[])
}

export async function createStation(lat: number, lon: number, name: string): Promise<Station> {
    const url = new URL(config.api_endpoint + "/el_add_station");
    url.searchParams.append("lat", String(lat))
    url.searchParams.append("lon", String(lon))
    url.searchParams.append("name", name)
    const response = await fetch(url);
    const data = await (response.ok ? response.json() : Promise.reject());
    return data as Station;
}

export async function rmStation(stop_id: number): Promise<void> {
    const url = new URL(config.api_endpoint + "/el_rm_station");
    url.searchParams.append("stop_id", String(stop_id))
    const response = await fetch(url);
    return await (response.ok ? Promise.resolve() : Promise.reject());
}

export async function moveStation(stop_id: number, lat: number, lon: number): Promise<Station> {
    const url = new URL(config.api_endpoint + "/el_move_station");
    url.searchParams.append("stop_id", String(stop_id))
    url.searchParams.append("lat", String(lat))
    url.searchParams.append("lon", String(lon))
    const response = await fetch(url);
    const data = await (response.ok ? response.json() : Promise.reject());
    return data as Station;
}

export async function addStationName(stop_id: number, name: string): Promise<Station> {
    const url = new URL(config.api_endpoint + "/el_add_station_name");
    url.searchParams.append("stop_id", String(stop_id))
    url.searchParams.append("name", name)
    const response = await fetch(url);
    const data = await (response.ok ? response.json() : Promise.reject());
    return data as Station;
}

export async function rmStationName(stop_id: number, name: string): Promise<Station> {
    const url = new URL(config.api_endpoint + "/el_rm_station_name");
    url.searchParams.append("stop_id", String(stop_id))
    url.searchParams.append("name", name)
    const response = await fetch(url);
    const data = await (response.ok ? response.json() : Promise.reject());
    return data as Station;
}
