import { useState, useEffect } from "react";
import { Link } from "react-router-dom";
import { Box, TextField, MenuItem } from "@mui/material";
import { LoadingButton } from "@mui/lab";
import { getAvailableTime, getCarsForSelf, postOrder } from "../api/api.js";
import { Car } from "../models";

type props = {
  serviceId: string;
};

const Book = ({ serviceId }: props) => {
  const [cars, setCars] = useState<Car[]>([]);
  const [availableTime, setAvailableTime] = useState<[string, string[]][]>([]);

  useEffect(() => {
    getAvailableTime(serviceId).then((data) => setAvailableTime(data));
    getCarsForSelf().then((data) => setCars(data));
  }, []);

  const [vin, setVin] = useState("");
  const [date, setDate] = useState("");
  const [time, setTime] = useState("");

  const [isLoading, setIsLoading] = useState(false);
  const [isDisabled, setIsDisabled] = useState(false);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setIsLoading(true);
    const car = cars.find((c) => c.vin === vin) || {
      vin: "",
      make: "",
      model: "",
      year: 0,
      contact: { id: "", phoneNumber: "", email: null },
    };

    postOrder({
      serviceId: serviceId,
      phoneNumber: car.contact.phoneNumber,
      email: car.contact.email || null,
      carVin: car.vin,
      carMake: car.make,
      carModel: car.model,
      carYear: car.year,
      startTime: date + " " + time,
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
      <TextField
        label="Car"
        select
        required
        value={vin}
        onChange={(e) => setVin(e.target.value)}
        margin="normal"
        sx={{ width: 0.916, mx: 4 }}
      >
        {cars.map((c, i) => (
          <MenuItem key={i} value={c.vin}>
            {c.vin + " ( " + c.make + " " + c.model + " " + c.year + " )"}
          </MenuItem>
        ))}
        <Link to="/cars/add">
          <MenuItem>ADD NEW</MenuItem>
        </Link>
      </TextField>
      <TextField
        label="Date"
        select
        required
        value={date}
        onChange={(e) => setDate(e.target.value)}
        margin="normal"
        sx={{ width: 0.916, mx: 4 }}
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
        sx={{ width: 0.916, mx: 4 }}
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
          disabled={isDisabled}
        >
          Confirm
        </LoadingButton>
      </Box>
    </form>
  );
};

export default Book;
