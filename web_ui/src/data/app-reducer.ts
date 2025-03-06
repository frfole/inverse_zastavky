import {CityRemap, Mode, Station} from "../model/model.ts";
import {AppState} from "./app-state.ts";
import {LatLng} from "leaflet";

export enum ActionType {
    ChangeMode,
    SetStations,
    UpdateStation,
    RemoveStation,
    MapClick,
    SearchStations,
    SetLocateOffset,
    SearchCity,
    SetCityRemap,
}

export interface ActionChangeMode {
    type: ActionType.ChangeMode,
    mode: Mode,
}

export interface ActionSetStations {
    type: ActionType.SetStations,
    new_stations: Station[],
}

export interface ActionUpdateStation {
    type: ActionType.UpdateStation,
    station: Station
}

export interface ActionRemoveStation {
    type: ActionType.RemoveStation,
    stop_id: number
}

export interface ActionMapClick {
    type: ActionType.MapClick,
    latLon: LatLng,
}

export interface ActionSearchStations {
    type: ActionType.SearchStations,
    query: string,
}

export interface ActionSetLocateOffset {
    type: ActionType.SetLocateOffset,
    newOffset: number,
}

export interface ActionSearchCity {
    type: ActionType.SearchCity,
    query: string,
}

export interface ActionSetCityRemap {
    type: ActionType.SetCityRemap,
    remap: CityRemap,
}

export type ReducerAction =
    ActionChangeMode
    | ActionSetStations
    | ActionUpdateStation
    | ActionRemoveStation
    | ActionMapClick
    | ActionSearchStations
    | ActionSetLocateOffset
    | ActionSearchCity
    | ActionSetCityRemap

export function appReducer(state: AppState, action: ReducerAction): AppState {
    switch (action.type) {
        case ActionType.ChangeMode: {
            return {
                ...state,
                mode: action.mode,
            }
        }
        case ActionType.SetStations: {
            return {
                ...state,
                stations: action.new_stations as Station[]
            }
        }
        case ActionType.UpdateStation: {
            const new_station = action.station as Station
            const stations = state.stations.map(station => {
                if (station.stop_id === new_station.stop_id) {
                    return new_station;
                } else {
                    return station;
                }
            })
            if (!stations.some(station => station.stop_id === new_station.stop_id)) {
                stations.push(new_station)
            }
            return {
                ...state,
                stations: stations
            }
        }
        case ActionType.RemoveStation: {
            const stations = state.stations.filter(station => station.stop_id !== action.stop_id)
            return {
                ...state,
                stations: stations
            }
        }
        case ActionType.MapClick: {
            return {
                ...state,
                lastMapClick: (action.latLon as LatLng).clone()
            }
        }
        case ActionType.SearchStations: {
            return {
                ...state,
                stationSearch: action.query
            }
        }
        case ActionType.SetLocateOffset: {
            return {
                ...state,
                locateOffset: action.newOffset
            }
        }
        case ActionType.SearchCity: {
            return {
                ...state,
                citySearch: action.query
            }
        }
        case ActionType.SetCityRemap: {
            return {
                ...state,
                cityRemap: action.remap
            }
        }
    }
}