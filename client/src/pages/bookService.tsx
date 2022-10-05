import { useParams } from "react-router-dom";
import { useState, useEffect } from "react";
import { getService, postOrder } from "../api/api.ts";

const BookService = () => {
  const { id } = useParams();
  const [service, setService] = useState({});
  const [phoneNumber, setPhoneNumber] = useState("");
  const [email, setEmail] = useState("");
  const [startTime, setStartTime] = useState("");
  const [carMake, setCarMake] = useState("");
  const [carModel, setCarModel] = useState("");
  const [carYear, setCarYear] = useState("");
  const [message, setMessage] = useState("");

  const handleSubmit = () => {
    postOrder({
      serviceId: service.id,
      phoneNumber: phoneNumber,
      email: email,
      startTime: startTime,
      carMake: carMake,
      carModel: carModel,
      carYear: carYear,
    }).then((res) => {
      let newMessage = res.ok
        ? "Successfully booked"
        : "An error occurred, try again later";
      setMessage(newMessage);
    });
  };

  useEffect(() => {
    getService(id).then((data) => setService(data));
  }, []);
  return (
    <div className="flex flex-col justify-center items-center my-10">
      <div className="flex flex-col justify-center items-center bg-gray-200 px-10 py-4 rounded">
        <div className="flex flex-col justify-center items-center">
          <div className="flex flex-col justify-center items-center">
            <p className="text-lg font-semibold mb-1">{service.title}</p>
            <p className="mb-1">price: {service.price}₴</p>
            <p className="mb-1">duration: {service.duration}</p>
          </div>
          {message === "" ? (
            <form className="flex justify-around items-around mt-3">
              <div className="flex flex-col justify-center items-center">
                <div className="flex flex-row justify-around items-center my-2">
                  <div className="w-1/2 mr-3">
                    <p className="text-center">Contacts</p>
                    <div className="flex-col flex justify-center items-center my-4">
                      <label
                        className="mt-3"
                        htmlFor="phoneNumber"
                        className="text-left"
                      >
                        Phone number
                      </label>
                      <input
                        id="phoneNumber"
                        type="text"
                        pattern="^[0-9]{10}$"
                        value={phoneNumber}
                        onChange={(e) => setPhoneNumber(e.target.value)}
                        placeholder="1234567890"
                        className="pl-2 mb-3 w-full rounded border hover:border-indigo-400"
                        required
                      />
                      <label
                        className="mt-3"
                        htmlFor="email"
                        className="text-left"
                      >
                        Email(optional)
                      </label>
                      <input
                        id="email"
                        type="text"
                        pattern="^[a-zA-Z]+@[a-zA-Z]+\.[a-zA-Z]+$"
                        value={email}
                        onChange={(e) => setEmail(e.target.value)}
                        placeholder="example@nure.ua"
                        className="pl-2 mb-3 w-full rounded border hover:border-indigo-400"
                      />
                      <label
                        className="mt-3"
                        htmlFor="startTime"
                        className="text-left"
                      >
                        Start time
                      </label>
                      <input
                        id="startTime"
                        type="text"
                        pattern="^[0-9]{2}\.[0-9]{2}\.20[0-9]{2} ([01]\d|2[0-3]):?([0-5]\d)$"
                        value={startTime}
                        onChange={(e) => setStartTime(e.target.value)}
                        placeholder="01.01.2000 00:00"
                        className="pl-2 mb-3 w-full rounded border hover:border-indigo-400"
                        required
                      />
                    </div>
                  </div>
                  <div className="w-1/2 ml-3">
                    <p className="text-center">Car information</p>
                    <div className="flex-col flex justify-center items-center my-4">
                      <label
                        className="mt-3"
                        htmlFor="carMake"
                        className="text-left"
                      >
                        Make
                      </label>
                      <input
                        id="carMake"
                        type="text"
                        pattern="^[a-zA-Z]+$"
                        value={carMake}
                        onChange={(e) => setCarMake(e.target.value)}
                        placeholder="Ferrari"
                        className="pl-2 mb-3 w-full rounded border hover:border-indigo-400"
                        required
                      />
                      <label
                        className="mt-3"
                        htmlFor="carModel"
                        className="text-left"
                      >
                        Model
                      </label>
                      <input
                        id="carModel"
                        type="text"
                        pattern="^[a-zA-Z]+$"
                        value={carModel}
                        onChange={(e) => setCarModel(e.target.value)}
                        placeholder="Testarossa"
                        className="pl-2 mb-3 w-full rounded border hover:border-indigo-400"
                      />
                      <label
                        className="mt-3"
                        htmlFor="carYear"
                        className="text-left"
                      >
                        Year
                      </label>
                      <input
                        id="carYear"
                        type="text"
                        pattern="^(19[0-9]{2}|20[0-2][0-9])$"
                        value={carYear}
                        onChange={(e) => setCarYear(e.target.value)}
                        placeholder="1988"
                        className="pl-2 mb-3 w-full rounded border hover:border-indigo-400"
                        required
                      />
                    </div>
                  </div>
                </div>
                <button
                  type="button"
                  onClick={handleSubmit}
                  className="text-lg hover:text-indigo-400"
                >
                  Confirm
                </button>
              </div>
            </form>
          ) : (
            <p className="mt-3 text-lg">{message}</p>
          )}
        </div>
      </div>
    </div>
  );
};

export default BookService;
