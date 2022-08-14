import { atom } from "recoil"
import { AuthStateType } from "../types/AuthStateType"

export const AuthStateAtom = atom<AuthStateType>({
  key: 'authState',
  default: {
    is_logged_in: false,
  },
})
