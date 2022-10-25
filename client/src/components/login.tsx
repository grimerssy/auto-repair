import React from "react";
import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { Box, TextField } from "@mui/material";
import { LoadingButton } from "@mui/lab";
import { login as postLogin } from "../api/api.js";

const newOnChange = (
  regex: RegExp,
  setValue: React.Dispatch<React.SetStateAction<string>>,
  setValid: React.Dispatch<React.SetStateAction<boolean>>
) => {
  return (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    let value = e.target.value;
    setValid(regex.test(value));
    setValue(value);
  };
};

type Tokens = {
  access: string;
};

const Login = () => {
  const [login, setLogin] = useState("");
  const [isLoginValid, setIsLoginValid] = useState(true);
  const [password, setPassword] = useState("");
  const [isPasswordValid, setIsPasswordValid] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [tokens, setTokens] = useState<Tokens>({ access: "" });
  const navigate = useNavigate();

  useEffect(() => {
    if (tokens.access) {
      localStorage.setItem("accessToken", tokens.access);
      navigate("/");
      window.location.reload();
    }
  }, [tokens]);

  const phoneNumberRegex = /^[0-9]{10}$/;
  const emailRegex = /^[a-zA-Z]+@[a-zA-Z]+\.[a-zA-Z]+$/;

  type apiParams = {
    phoneNumber: string | null;
    email: string | null;
    password: string;
  };

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setIsLoading(true);
    const params = {
      phoneNumber: null,
      email: null,
      password: password,
    } as apiParams;
    if (phoneNumberRegex.test(login)) {
      params.phoneNumber = login;
    } else if (emailRegex.test(login)) {
      params.email = login;
    }
    postLogin(params).then((tokens) => {
      setTokens(tokens);
      setIsLoading(false);
    });
  };

  return (
    <form onSubmit={handleSubmit}>
      <Box sx={{ display: "flex", flexDirection: "column" }}>
        <TextField
          label="Phone number or email"
          required
          value={login}
          error={!isLoginValid}
          onChange={newOnChange(
            new RegExp(
              "(" + phoneNumberRegex.source + "|" + emailRegex.source + ")"
            ),
            setLogin,
            setIsLoginValid
          )}
          margin="normal"
        />
        <TextField
          label="password"
          required
          type="password"
          value={password}
          error={!isPasswordValid}
          onChange={newOnChange(
            // /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[a-zA-Z\d]{8,64}$/,
            /.+/,
            setPassword,
            setIsPasswordValid
          )}
          margin="normal"
        />
      </Box>
      <Box textAlign="center">
        <LoadingButton
          type="submit"
          variant="text"
          loading={isLoading}
          sx={{ mt: 4 }}
          disabled={[isLoginValid, isPasswordValid].some((v) => !v)}
        >
          Log in
        </LoadingButton>
      </Box>
    </form>
  );
};

export default Login;
