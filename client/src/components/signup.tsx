import React from "react";
import { useState } from "react";
import { Box, TextField } from "@mui/material";
import { LoadingButton } from "@mui/lab";
import { signup as postSignup } from "../api/api.js";

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

const Signup = () => {
  const [phoneNumber, setPhoneNumber] = useState("");
  const [isPhoneNumberValid, setIsPhoneNumberValid] = useState(true);
  const [email, setEmail] = useState("");
  const [isEmailValid, setIsEmailValid] = useState(true);
  const [password, setPassword] = useState("");
  const [isPasswordValid, setIsPasswordValid] = useState(true);
  const [firstName, setFirstName] = useState("");
  const [isFirstNameValid, setIsFirstNameValid] = useState(true);
  const [middleName, setMiddleName] = useState("");
  const [isMiddleNameValid, setIsMiddleNameValid] = useState(true);
  const [lastName, setLastName] = useState("");
  const [isLastNameValid, setIsLastNameValid] = useState(true);
  const [dateOfBirth, setDateOfBirth] = useState("");
  const [isDateOfBirthValid, setIsDateOfBirthValid] = useState(true);
  const [isLoading, setIsLoading] = useState(false);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setIsLoading(true);
    const params = {
      phoneNumber: phoneNumber,
      email: email || null,
      password: password,
      firstName: firstName,
      middleName: middleName || null,
      lastName: lastName,
      dateOfBirth: dateOfBirth,
    };
    postSignup(params).then(() => {
      setIsLoading(false);
      alert("Signed up successfully. Now go to the login page");
    });
  };

  return (
    <form onSubmit={handleSubmit}>
      <Box sx={{ display: "flex", flexDirection: "column" }}>
        <TextField
          label="Phone number"
          required
          value={phoneNumber}
          error={!isPhoneNumberValid}
          onChange={newOnChange(
            /^[0-9]{10}$/,
            setPhoneNumber,
            setIsPhoneNumberValid
          )}
          margin="normal"
        />
        <TextField
          label="Email"
          value={email}
          error={!isEmailValid}
          onChange={newOnChange(
            /^(|[a-zA-Z]+@[a-zA-Z]+\.[a-zA-Z]+)$/,
            setEmail,
            setIsEmailValid
          )}
          margin="normal"
        />
        <TextField
          label="Password"
          required
          type="password"
          value={password}
          error={!isPasswordValid}
          onChange={newOnChange(
            /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[a-zA-Z\d]{8,64}$/,
            setPassword,
            setIsPasswordValid
          )}
          margin="normal"
        />
        <TextField
          label="First name"
          required
          value={firstName}
          error={!isFirstNameValid}
          onChange={newOnChange(
            /^[A-Z][a-z]+$/,
            setFirstName,
            setIsFirstNameValid
          )}
          margin="normal"
        />
        <TextField
          label="Middle name"
          value={middleName}
          error={!isMiddleNameValid}
          onChange={newOnChange(
            /^(|[A-Z][a-z]+)$/,
            setMiddleName,
            setIsMiddleNameValid
          )}
          margin="normal"
        />
        <TextField
          label="Last name"
          required
          value={lastName}
          error={!isLastNameValid}
          onChange={newOnChange(
            /^[A-Z][a-z]+$/,
            setLastName,
            setIsLastNameValid
          )}
          margin="normal"
        />
        <TextField
          label="Date of birth"
          required
          value={dateOfBirth}
          error={!isDateOfBirthValid}
          onChange={newOnChange(
            /^(0[1-9]|[1-3]\d)\.(0[1-9]|1[0-2])\.(19|20)[0-9]{2}$/,
            setDateOfBirth,
            setIsDateOfBirthValid
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
          disabled={[
            isPhoneNumberValid,
            isEmailValid,
            isPasswordValid,
            isFirstNameValid,
            isMiddleNameValid,
            isLastNameValid,
            isDateOfBirthValid,
          ].some((v) => !v)}
        >
          sign up
        </LoadingButton>
      </Box>
    </form>
  );
};

export default Signup;
