import {BaseCity, BaseStation, BBox, ChainStation, Station} from "../model/model.ts";
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

export async function getChainStations(offset: number, limit: number): Promise<ChainStation[]> {
    const url = new URL(config.api_endpoint + "/sl_chains");
    url.searchParams.append("offset", String(offset))
    url.searchParams.append("limit", String(limit))
    const response = await fetch(url);
    const data = await (response.ok ? response.json() : Promise.reject());
    return data as ChainStation[];
}

export async function locateByLoc(chainStation: ChainStation, lat: number, lon: number): Promise<Station> {
    const url = new URL(config.api_endpoint + "/locate_by_loc");
    url.searchParams.append("chain_hash", String(chainStation.chain_hash))
    url.searchParams.append("name", String(chainStation.name))
    url.searchParams.append("pos", String(chainStation.pos))
    url.searchParams.append("lat", String(lat))
    url.searchParams.append("lon", String(lon))
    const response = await fetch(url);
    const data = await (response.ok ? response.json() : Promise.reject());
    return data as Station;
}

export async function locateById(chainStation: ChainStation, station: Station): Promise<Station> {
    const url = new URL(config.api_endpoint + "/locate_by_id");
    url.searchParams.append("chain_hash", String(chainStation.chain_hash))
    url.searchParams.append("name", String(chainStation.name))
    url.searchParams.append("pos", String(chainStation.pos))
    url.searchParams.append("stop_id", String(station.stop_id))
    const response = await fetch(url);
    const data = await (response.ok ? response.json() : Promise.reject());
    return data as Station;
}

export async function getBaseStations(bbox: BBox): Promise<BaseStation[]> {
    const url = new URL(config.api_endpoint + "/sl_base_stations");
    url.searchParams.append("lat_from", String(bbox.lat_from))
    url.searchParams.append("lat_to", String(bbox.lat_to))
    url.searchParams.append("lon_from", String(bbox.lon_from))
    url.searchParams.append("lon_to", String(bbox.lon_to))
    return await fetch(url)
        .then(response => response.ok ? response.json() : Promise.reject())
        .then(data => data as BaseStation[])
}

export async function searchBaseCity(query: string): Promise<BaseCity[]> {
    const url = new URL(config.api_endpoint + "/sl_base_city");
    url.searchParams.append("query", query)
    return await fetch(url)
        .then(response => response.ok ? response.json() : Promise.reject())
        .then(data => data as BaseCity[])
}
