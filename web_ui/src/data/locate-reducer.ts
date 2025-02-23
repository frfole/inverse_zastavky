import {LocateState} from "./locate-state.ts";
import {BaseStation, ChainStation} from "../model/model.ts";

export enum LocateActionType {
    SetChainStations,
    SetBaseStations,
    SetOffset,
    SetSelectedIdx,
    Advance,
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

export type LocateAction =
    ActionSetChainStations
    | ActionSetBaseStations
    | ActionSetOffset
    | ActionSetSelectedIdx
    | ActionAdvance

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
    }
}
