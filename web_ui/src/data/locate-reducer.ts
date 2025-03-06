import {LocateState} from "./locate-state.ts";
import {BaseStation, ChainCitiesSuggestion, ChainStation, ChainStationsSuggestion} from "../model/model.ts";
import {LatLngExpression} from "leaflet";

export enum LocateActionType {
    SetChainStations,
    SetBaseStations,
    SetOffset,
    SetSelectedIdx,
    Advance,
    SetStationsSuggestions,
    SetCitiesSuggestions,
    SetSuggestionPreview,
    Reload,
}

export interface ActionSetChainStations {
    type: LocateActionType.SetChainStations,
    newStations: ChainStation[]
}

export interface ActionSetBaseStations {
    type: LocateActionType.SetBaseStations,
    newStations: BaseStation[]
}

export interface ActionSetOffset {
    type: LocateActionType.SetOffset,
    offset: number,
}

export interface ActionSetSelectedIdx {
    type: LocateActionType.SetSelectedIdx,
    idx: number,
}

export interface ActionAdvance {
    type: LocateActionType.Advance,
}

export interface ActionSetStationsSuggestions {
    type: LocateActionType.SetStationsSuggestions,
    newSuggestions: ChainStationsSuggestion[],
}

export interface ActionSetCitiesSuggestions {
    type: LocateActionType.SetCitiesSuggestions,
    newSuggestions: ChainCitiesSuggestion[],
}

export interface ActionSetSuggestionPreview {
    type: LocateActionType.SetSuggestionPreview,
    path?: LatLngExpression[],
}

export interface ActionReload {
    type: LocateActionType.Reload,
}

export type LocateAction =
    ActionSetChainStations
    | ActionSetBaseStations
    | ActionSetOffset
    | ActionSetSelectedIdx
    | ActionAdvance
    | ActionSetStationsSuggestions
    | ActionSetCitiesSuggestions
    | ActionSetSuggestionPreview
    | ActionReload

export function locateReducer(state: LocateState, action: LocateAction): LocateState {
    switch (action.type) {
        case LocateActionType.SetChainStations:
            return {
                ...state,
                chainStations: action.newStations
            }
        case LocateActionType.SetBaseStations:
            return {
                ...state,
                baseStations: action.newStations
            }
        case LocateActionType.SetOffset: {
            const actualOffset = Math.max(0, action.offset - state.limit * 0.2)
            const idx = action.offset - actualOffset
            return {
                ...state,
                offset: actualOffset,
                selectedIdx: idx,
            }
        }
        case LocateActionType.SetSelectedIdx:
            return {
                ...state,
                selectedIdx: action.idx
            }
        case LocateActionType.Advance:
            if (state.selectedIdx < state.limit * 0.2) {
                return {
                    ...state,
                    selectedIdx: state.selectedIdx + 1,
                    offset: state.offset
                }
            } else if (state.selectedIdx > state.limit * 0.4) {
                return {
                    ...state,
                    selectedIdx: state.selectedIdx - 1,
                    offset: state.offset + 2
                }
            } else {
                return {
                    ...state,
                    selectedIdx: state.selectedIdx,
                    offset: state.offset + 1
                }
            }
        case LocateActionType.SetStationsSuggestions:
            return {
                ...state,
                stationsSuggestions: action.newSuggestions,
            }
        case LocateActionType.SetCitiesSuggestions:
            return {
                ...state,
                citiesSuggestions: action.newSuggestions,
            }
        case LocateActionType.SetSuggestionPreview:
            return {
                ...state,
                suggestionPreview: action.path,
            }
        case LocateActionType.Reload:
            return {
                ...state,
                reload: !state.reload
            }
    }
}
