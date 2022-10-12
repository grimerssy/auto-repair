import { useEffect, useState } from "react";
import { postOrder } from "../api/api.ts";
import { TextField, Button, Typography } from "@mui/material";

const BookNoAuth = ({ serviceId }) => {
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

  const handleSubmit = (event) => {
    event.preventDefault();
    postOrder({
      serviceId: serviceId,
      phoneNumber: phoneNumber,
      email: email,
      startTime: startTime,
      carMake: carMake,
      carModel: carModel,
      carYear: carYear,
    }).then((res) => {
      let msg = res.ok ? "Booked successfully" : "An error occurred. Try again";
      alert(msg);
    });
  };

  return (
    <form
      onSubmit={handleSubmit}
      className="flex justify-around items-around mt-3"
    >
      <div className="flex flex-col justify-center items-center">
        <div className="flex flex-row justify-around items-center my-2">
          <div className="w-1/2 mr-3">
            <Typography variant="h6" align="center">
              Contacts
            </Typography>
            <div className="flex-col flex justify-center items-center my-4">
              <TextField
                label="Phone number"
                required
                value={phoneNumber}
                error={!isPhoneNumberValid}
                onChange={(e) => {
                  const regex = /^[0-9]{10}$/;
                  let value = e.target.value;
                  setIsPhoneNumberValid(regex.test(value));
                  setPhoneNumber(value);
                }}
                color="secondary"
                margin="normal"
              />
              <TextField
                label="Email"
                value={email}
                error={!isEmailValid}
                onChange={(e) => {
                  const regex = /^(|[a-zA-Z]+@[a-zA-Z]+\.[a-zA-Z]+)$/;
                  let value = e.target.value;
                  setIsEmailValid(regex.test(value));
                  setEmail(value);
                }}
                color="secondary"
                margin="normal"
              />
              <TextField
                label="Start time"
                required
                value={startTime}
                error={!isStartTimeValid}
                onChange={(e) => {
                  const regex =
                    /^([1-9]|[1-3]\d)\.([1-9]|1[0-2])\.20[0-9]{2} (1?\d|2[0-3]):?([0-5]\d)$/;
                  let value = e.target.value;
                  setIsStartTimeValid(regex.test(value));
                  setStartTime(value);
                }}
                color="secondary"
                margin="normal"
              />
            </div>
          </div>
          <div className="w-1/2 ml-3">
            <Typography variant="h6" align="center">
              Car information
            </Typography>
            <div className="flex-col flex items-center my-4">
              <TextField
                label="Make"
                required
                value={carMake}
                error={!isCarMakeValid}
                onChange={(e) => {
                  const regex = /^[a-zA-Z]+$/;
                  let value = e.target.value;
                  setIsCarMakeValid(regex.test(value));
                  setCarMake(value);
                }}
                color="secondary"
                margin="normal"
              />
              <TextField
                label="Model"
                required
                value={carModel}
                error={!isCarModelValid}
                onChange={(e) => {
                  const regex = /^[a-zA-Z]+$/;
                  let value = e.target.value;
                  setIsCarModelValid(regex.test(value));
                  setCarModel(value);
                }}
                color="secondary"
                margin="normal"
              />
              <TextField
                label="Year"
                required
                value={carYear}
                error={!isCarYearValid}
                onChange={(e) => {
                  const regex = /^(19[5-9][0-9]|20([01][0-9]|2[0-2]))$/;
                  let value = e.target.value;
                  setIsCarYearValid(regex.test(value));
                  setCarYear(value);
                }}
                color="secondary"
                margin="normal"
              />
            </div>
          </div>
        </div>
        <Button label="text" type="submit" variant="text" color="secondary">
          Confirm
        </Button>
      </div>
    </form>
  );
};

export default BookNoAuth;
