import {Mode, Station} from "../model/model.ts";
import {LatLng} from "leaflet";

export interface AppState {
    mode: Mode,
    stations: Station[],
    lastMapClick: LatLng | null,
    stationSearch: string,
    citySearch: string,
    locateOffset: number,
}