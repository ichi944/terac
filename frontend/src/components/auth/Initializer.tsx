import React, { useEffect } from "react";
import { useRecoilValue, useSetRecoilState } from "recoil";
import { InitializeStateAtom } from "../../atoms/InitializeStateAtom";

export const Initializer = () => {
  const initializeState = useRecoilValue(InitializeStateAtom);
  const setInitializeState = useSetRecoilState(InitializeStateAtom);
  useEffect(() => {
    fetch("/api/hello", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
    })
      .then((res) => res.json())
      .then((json) => {
        console.log(json);
        setInitializeState({ initialized: true });
      }).catch(() => {
        console.log('error for hello api');
        setInitializeState({ initialized: true });

      });
  }, [initializeState.initialized]);

  return <div>...initializing</div>;
};
