import React from "react";
import { useState, useEffect } from "react";
import { Box, TextField, Divider, MenuItem } from "@mui/material";
import { LoadingButton } from "@mui/lab";
import { getAvailableTime, updateOrder } from "../api/api";
import { Order } from "../models";

type props = {
  order: Order;
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

const EditOrder = ({ order }: props) => {
  const [availableTime, setAvailableTime] = useState<[string, string[]][]>([]);

  useEffect(() => {
    getAvailableTime(order.service.id).then((data) => setAvailableTime(data));
  }, []);

  const [phoneNumber, setPhoneNumber] = useState(order.car.contact.phoneNumber);
  const [isPhoneNumberValid, setIsPhoneNumberValid] = useState(true);
  const [email, setEmail] = useState(order.car.contact.email || "");
  const [isEmailValid, setIsEmailValid] = useState(true);
  const [carVin, setCarVin] = useState(order.car.vin);
  const [isCarVinValid, setIsCarVinValid] = useState(true);
  const [carMake, setCarMake] = useState(order.car.make);
  const [isCarMakeValid, setIsCarMakeValid] = useState(true);
  const [carModel, setCarModel] = useState(order.car.model);
  const [isCarModelValid, setIsCarModelValid] = useState(true);
  const [carYear, setCarYear] = useState(order.car.year + "");
  const [isCarYearValid, setIsCarYearValid] = useState(true);
  const [date, setDate] = useState(order.startTime.split(" ")[0]);
  const [time, setTime] = useState(order.startTime.split(" ")[1]);

  const [isLoading, setIsLoading] = useState(false);
  const [isDisabled, setIsDisabled] = useState(false);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setIsLoading(true);
    updateOrder({
      id: order.id,
      serviceId: order.service.id,
      phoneNumber: phoneNumber,
      email: email || null,
      carVin: carVin,
      carMake: carMake,
      carModel: carModel,
      carYear: +carYear,
      startTime: date + " " + time,
    }).then((res) => {
      const msg = res.ok
        ? "Updated successfully"
        : "An error occurred. Try again";
      alert(msg);
      setIsLoading(false);
      setIsDisabled(true);
    });
  };

  return (
    <form onSubmit={handleSubmit}>
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
        sx={{ width: 0.458, mx: 2 }}
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
        sx={{ width: 0.458, mx: 2 }}
      />
      <Divider sx={{ m: 2 }} />
      <TextField
        label="Vin number"
        required
        value={carVin}
        error={!isCarVinValid}
        onChange={newOnChange(
          /^(?=.*[0-9])(?=.*[A-z])[0-9A-z-]{17}$/,
          setCarVin,
          setIsCarVinValid
        )}
        margin="normal"
        sx={{ width: 0.458, mx: 2 }}
      />
      <TextField
        label="Make"
        required
        value={carMake}
        error={!isCarMakeValid}
        onChange={newOnChange(/^[a-zA-Z]+$/, setCarMake, setIsCarMakeValid)}
        margin="normal"
        sx={{ width: 0.458, mx: 2 }}
      />
      <TextField
        label="Model"
        required
        value={carModel}
        error={!isCarModelValid}
        onChange={newOnChange(
          /^[0-9a-zA-Z\- ]+$/,
          setCarModel,
          setIsCarModelValid
        )}
        margin="normal"
        sx={{ width: 0.458, mx: 2 }}
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
        sx={{ width: 0.458, mx: 2 }}
      />
      <Divider sx={{ m: 2 }} />
      <TextField
        label="Date"
        select
        required
        value={date}
        onChange={(e) => setDate(e.target.value)}
        margin="normal"
        sx={{ width: 0.458, mx: 2 }}
      >
        {availableTime.map((dt, i) => (
          <MenuItem key={i} value={dt[0]}>
            {dt[0]}
          </MenuItem>
        ))}
      </TextField>
      <TextField
        label="Time"
        select
        required
        value={time}
        onChange={(e) => setTime(e.target.value)}
        margin="normal"
        sx={{ width: 0.458, mx: 2 }}
      >
        {(availableTime.find((dt) => dt[0] === date) || ["", []])[1].map(
          (t, i) => (
            <MenuItem key={i} value={t}>
              {t}
            </MenuItem>
          )
        )}
      </TextField>
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
              isCarVinValid,
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

export default EditOrder;
