import {BaseStation, ChainStation, ChainStationsSuggestion} from "../model/model.ts";
import {LatLngExpression} from "leaflet";

export interface LocateState {
    offset: number,
    limit: number,
    selectedIdx: number,
    chainStations: ChainStation[],
    baseStations: BaseStation[],
    stationsSuggestions: ChainStationsSuggestion[],
    suggestionPreview?: LatLngExpression[],
    reload: boolean
}
