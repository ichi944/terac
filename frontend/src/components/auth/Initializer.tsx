import React, { useEffect } from "react";
import { useRecoilValue, useSetRecoilState } from "recoil";
import { InitializeStateAtom } from "../../atoms/InitializeStateAtom";
import { AuthStateAtom } from "../../atoms/AuthStateAtom";

export const Initializer = () => {
  const initializeState = useRecoilValue(InitializeStateAtom);
  const setInitializeState = useSetRecoilState(InitializeStateAtom);
  const setAuthStete = useSetRecoilState(AuthStateAtom);
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
        if (json.is_logged_in) {
          setAuthStete({is_logged_in: true});
        }
      }).catch(() => {
        console.log('error for hello api');
        setInitializeState({ initialized: false });

      });
  }, [initializeState.initialized]);

  return <div>...initializing</div>;
};
