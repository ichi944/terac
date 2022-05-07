import React from "react";
import {
  RecoilRoot,
  atom,
  selector,
  useRecoilState,
  useRecoilValue,
  useSetRecoilState,
} from "recoil";
import { DrawerStateAtom } from "../atoms/DrawerStateAtom";
import { AppDrawer } from "./AppDrawer";

export const Home = () => {
  const drawerIsOpen = useRecoilValue(DrawerStateAtom);
  const setDrawerState = useSetRecoilState(DrawerStateAtom);
  const handleToggleDrawer = () => {
    setDrawerState((state) => ({ ...state, isOpen: !state.isOpen }));
  };
  const onClose = () => {
    setDrawerState((state) => ({ ...state, isOpen: false }));
  };
  return (
    <>
      <AppDrawer isOpen={drawerIsOpen.isOpen} onClose={onClose} />
      Home Screen
      <button onClick={handleToggleDrawer}>Toggle!</button>

    </>
  );
};
