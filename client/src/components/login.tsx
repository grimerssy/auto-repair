import React from "react";
import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { Box, TextField } from "@mui/material";
import { LoadingButton } from "@mui/lab";
import { login as postLogin } from "../api/api.js";

type Tokens = {
  access: string;
};

const Login = () => {
  const [login, setLogin] = useState("");
  const [password, setPassword] = useState("");
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
    if (login.includes("@")) {
      params.email = login;
    } else {
      params.phoneNumber = login;
    }
    postLogin(params).then((tokens) => {
      if (tokens.access) {
        setTokens(tokens);
        setIsLoading(false);
      } else {
        alert("Invalid login or password");
        setIsLoading(false);
      }
    });
  };

  return (
    <form onSubmit={handleSubmit}>
      <Box sx={{ display: "flex", flexDirection: "column" }}>
        <TextField
          label="Phone number or email"
          required
          value={login}
          onChange={(e) => setLogin(e.target.value)}
          margin="normal"
        />
        <TextField
          label="Password"
          required
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          margin="normal"
        />
      </Box>
      <Box textAlign="center">
        <LoadingButton
          type="submit"
          variant="text"
          loading={isLoading}
          sx={{ mt: 4 }}
        >
          Log in
        </LoadingButton>
      </Box>
    </form>
  );
};

export default Login;
