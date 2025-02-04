import {Mode, Station} from "../model/model.ts";
import {AppState} from "./app-state.ts";
import {LatLng} from "leaflet";

export enum ActionType {
    ModeChange,
    SetStations,
    UpdateStation,
    RemoveStation,
    MapClick,
}

export interface ReducerAction {
    type: ActionType
    mode?: Mode
    new_stations?: Station[],
    stop_id?: number,
    station?: Station,
    latLon?: LatLng,
}

export function appReducer(state: AppState, action: ReducerAction): AppState {
    switch (action.type) {
        case ActionType.ModeChange: {
            return {
                ...state,
                mode: action.mode as Mode,
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
    }
}