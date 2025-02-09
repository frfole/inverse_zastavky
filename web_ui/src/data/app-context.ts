import {ActionDispatch, createContext} from "react";
import {ReducerAction} from "./app-reducer.ts";
import {AppState} from "./app-state.ts";

export const AppContext = createContext<AppState>(null)
export const AppDispatchContext
    = createContext<ActionDispatch<[action: ReducerAction]>>(null)
