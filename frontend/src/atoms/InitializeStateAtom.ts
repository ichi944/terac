import { atom } from "recoil"
import { InitializeStateType } from "../types/InitializeStateType"

export const InitializeStateAtom = atom<InitializeStateType>({
  key: 'initializeState',
  default: {
    initialized: false,
  },
})
