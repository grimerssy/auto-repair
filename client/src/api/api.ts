const server_host = "http://127.0.0.1:8080/api";

export const getAllServices = async () => {
  const url = server_host + "/services";
  const res = await fetch(url);
  return await res.json();
};

export const getService = async (id: string) => {
  const url = server_host + "/services/" + id;
  const res = await fetch(url);
  return await res.json();
};

export const postOrder = (order: {
  serviceId: string;
  phoneNumber: string;
  email: string | null;
  startTime: string;
  carMake: string;
  carModel: string;
  carYear: number;
}) => {
  const url = server_host + "/orders";
  let obj = {
    serviceId: order.serviceId,
    phoneNumber: order.phoneNumber,
    email: order.email,
    startTime: order.startTime,
    carMake: order.carMake,
    carModel: order.carModel,
    carYear: order.carYear,
  };
  return fetch(url, {
    method: "POST",
    body: JSON.stringify(obj),
    headers: {
      "Content-Type": "application/json",
    },
  });
};

export const getAllOrders = async () => {
  const url = server_host + "/orders";
  const res = await fetch(url);
  return await res.json();
};

export const getOrdersByServiceId = async (id: string) => {
  const url = server_host + "/orders/" + id;
  const res = await fetch(url);
  return await res.json();
};
