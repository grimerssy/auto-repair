const serverHost = "http://127.0.0.1:8080";
const baseUrl = serverHost + "/api";

export const login = async (creds: {
  phoneNumber: string | null;
  email: string | null;
  password: string;
}) => {
  const url = baseUrl + "/auth/login";
  const res = await fetch(url, {
    method: "POST",
    body: JSON.stringify(creds),
    headers: {
      "Content-Type": "application/json",
    },
  });
  return await res.json();
};

export const getAllServices = async () => {
  const url = baseUrl + "/services";
  const res = await fetch(url);
  return await res.json();
};

export const getService = async (id: string) => {
  const url = baseUrl + "/services/" + id;
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
  const url = baseUrl + "/orders";
  return fetch(url, {
    method: "POST",
    body: JSON.stringify(order),
    headers: {
      "Content-Type": "application/json",
    },
  });
};

export const updateOrder = (order: {
  id: string;
  serviceId: string;
  phoneNumber: string;
  email: string | null;
  startTime: string;
  carMake: string;
  carModel: string;
  carYear: number;
}) => {
  const url = baseUrl + "/orders/update/" + order.id;
  return fetch(url, {
    method: "PUT",
    body: JSON.stringify({
      serviceId: order.serviceId,
      phoneNumber: order.phoneNumber,
      email: order.email,
      startTime: order.startTime,
      carMake: order.carMake,
      carModel: order.carModel,
      carYear: order.carYear,
    }),
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

export const getAllOrders = async () => {
  const url = baseUrl + "/orders";
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};

export const getOrderById = async (id: string) => {
  const url = baseUrl + "/orders/" + id;
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};

export const getOrdersByServiceId = async (id: string) => {
  const url = baseUrl + "/orders/service/" + id;
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};

export const deleteOrderById = async (id: string) => {
  const url = baseUrl + "/orders/" + id;
  const res = await fetch(url, {
    method: "DELETE",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};
