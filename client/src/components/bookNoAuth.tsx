import React from "react";
import { useState } from "react";
import { Box, TextField, Grid } from "@mui/material";
import { LoadingButton } from "@mui/lab";
import { postOrder } from "../api/api.js";

type props = {
  serviceId: string;
};

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

const BookNoAuth = ({ serviceId }: props) => {
  const [phoneNumber, setPhoneNumber] = useState("");
  const [isPhoneNumberValid, setIsPhoneNumberValid] = useState(true);
  const [email, setEmail] = useState("");
  const [isEmailValid, setIsEmailValid] = useState(true);
  const [startTime, setStartTime] = useState("");
  const [isStartTimeValid, setIsStartTimeValid] = useState(true);
  const [carMake, setCarMake] = useState("");
  const [isCarMakeValid, setIsCarMakeValid] = useState(true);
  const [carModel, setCarModel] = useState("");
  const [isCarModelValid, setIsCarModelValid] = useState(true);
  const [carYear, setCarYear] = useState("");
  const [isCarYearValid, setIsCarYearValid] = useState(true);
  const [isLoading, setIsLoading] = useState(false);
  const [isDisabled, setIsDisabled] = useState(false);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setIsLoading(true);
    postOrder({
      serviceId: serviceId,
      phoneNumber: phoneNumber,
      email: email || null,
      startTime: startTime,
      carMake: carMake,
      carModel: carModel,
      carYear: +carYear,
    }).then((res) => {
      const msg = res.ok
        ? "Booked successfully"
        : "An error occurred. Try again";
      alert(msg);
      setIsLoading(false);
      setIsDisabled(true);
    });
  };

  return (
    <form onSubmit={handleSubmit}>
      <Grid container spacing={4}>
        <Grid item xs={6} sx={{ display: "flex", flexDirection: "column" }}>
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
            label="Start time"
            required
            value={startTime}
            error={!isStartTimeValid}
            onChange={newOnChange(
              /^(0[1-9]|[1-3]\d)\.(0[1-9]|1[0-2])\.20[0-9]{2} ((1|0?)\d|2[0-3]):([0-5]\d)$/,
              setStartTime,
              setIsStartTimeValid
            )}
            margin="normal"
          />
        </Grid>
        <Grid item xs={6} sx={{ display: "flex", flexDirection: "column" }}>
          <TextField
            label="Make"
            required
            value={carMake}
            error={!isCarMakeValid}
            onChange={newOnChange(/^[a-zA-Z]+$/, setCarMake, setIsCarMakeValid)}
            margin="normal"
          />
          <TextField
            label="Model"
            required
            value={carModel}
            error={!isCarModelValid}
            onChange={newOnChange(
              /^[a-zA-Z]+$/,
              setCarModel,
              setIsCarModelValid
            )}
            margin="normal"
          />
          <TextField
            label="Year"
            required
            value={carYear}
            error={!isCarYearValid}
            onChange={newOnChange(
              /^(19[5-9][0-9]|20([01][0-9]|2[0-2]))$/,
              setCarYear,
              setIsCarYearValid
            )}
            margin="normal"
          />
        </Grid>
      </Grid>
      <Box textAlign="center">
        <LoadingButton
          type="submit"
          variant="text"
          loading={isLoading}
          sx={{ mt: 4 }}
          disabled={
            isDisabled ||
            [
              isPhoneNumberValid,
              isEmailValid,
              isStartTimeValid,
              isCarMakeValid,
              isCarYearValid,
              isCarYearValid,
            ].some((v) => !v)
          }
        >
          Confirm
        </LoadingButton>
      </Box>
    </form>
  );
};

export default BookNoAuth;
