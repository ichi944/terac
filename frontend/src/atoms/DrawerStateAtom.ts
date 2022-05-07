import { atom } from "recoil"
import { DrawerStateType } from "../types/DrawerStateType"

export const DrawerStateAtom = atom<DrawerStateType>({
  key: 'drawerState',
  default: {
    isOpen: false,
  },
})
