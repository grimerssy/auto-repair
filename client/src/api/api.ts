const server_host = "http://127.0.0.1:8080/api";

export const getAllServices = () => {
  const url = server_host + "/services";
  return fetch(url).then((res) => {
    return res.json();
  });
};

export const getService = (id) => {
  const url = server_host + "/services/" + id;
  return fetch(url).then((res) => {
    return res.json();
  });
};

export const postOrder = ({
  serviceId,
  phoneNumber,
  email,
  startTime,
  carMake,
  carModel,
  carYear,
}) => {
  const url = server_host + "/orders";
  let obj = {
    serviceId: serviceId,
    phoneNumber: phoneNumber,
    startTime: startTime,
    carMake: carMake,
    carModel: carModel,
    carYear: +carYear,
  };
  if (email !== "") {
    obj.email = email;
  }
  console.log(obj);
  return fetch(url, {
    method: "POST",
    body: JSON.stringify(obj),
    headers: {
      "Content-Type": "application/json",
    },
  });
};
