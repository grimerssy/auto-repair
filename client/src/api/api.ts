const serverHost = "http://127.0.0.1:8080";
const baseUrl = serverHost + "/api";

export const executeSql = async (query: string) => {
  const url = baseUrl + "/sql";
  const res = await fetch(url, {
    method: "POST",
    body: JSON.stringify({ query: query }),
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  if (res.ok) {
    try {
      return await res.json();
    } catch {
      return "OK";
    }
  }
  return "An error occurred, try again later";
};

export const createCar = (car: {
  vin: string;
  make: string;
  model: string;
  year: number;
}) => {
  const url = baseUrl + "/cars";
  return fetch(url, {
    method: "POST",
    body: JSON.stringify(car),
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

export const createCarForSelf = (car: {
  vin: string;
  make: string;
  model: string;
  year: number;
}) => {
  const url = baseUrl + "/cars/self";
  return fetch(url, {
    method: "POST",
    body: JSON.stringify(car),
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

export const updateCarByVin = (
  vin: string,
  car: {
    vin: string;
    make: string;
    model: string;
    year: number;
  }
) => {
  const url = baseUrl + "/cars/" + vin;
  return fetch(url, {
    method: "PUT",
    body: JSON.stringify(car),
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

export const updateCarForSelf = (
  vin: string,
  car: {
    vin: string;
    make: string;
    model: string;
    year: number;
  }
) => {
  const url = baseUrl + "/cars/self/" + vin;
  return fetch(url, {
    method: "PUT",
    body: JSON.stringify(car),
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

export const getCarByVin = async (vin: string) => {
  const url = baseUrl + "/cars/" + vin;
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};

export const getCarByVinForSelf = async (vin: string) => {
  const url = baseUrl + "/cars/self/" + vin;
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};

export const getAllCars = async () => {
  const url = baseUrl + "/cars";
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};

export const getCarsForSelf = async () => {
  const url = baseUrl + "/cars/self";
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};

export const deleteCarByVin = (vin: string) => {
  const url = baseUrl + "/cars/" + vin;
  return fetch(url, {
    method: "DELETE",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

export const deleteCarForSelf = (vin: string) => {
  const url = baseUrl + "/cars/self/" + vin;
  return fetch(url, {
    method: "DELETE",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

export const getAllContacts = async () => {
  const url = baseUrl + "/contacts";
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};

export const getContact = async () => {
  const url = baseUrl + "/contacts/self";
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};

export const getContactById = async (id: string) => {
  const url = baseUrl + "/contacts/" + id;
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};

export const updateContact = (contact: {
  phoneNumber: string;
  email: string | null;
}) => {
  const url = baseUrl + "/contacts/self";
  return fetch(url, {
    method: "PUT",
    body: JSON.stringify(contact),
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

export const updateContactById = (
  id: string,
  contact: {
    phoneNumber: string;
    email: string | null;
  }
) => {
  const url = baseUrl + "/contacts/" + id;
  return fetch(url, {
    method: "PUT",
    body: JSON.stringify(contact),
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

export const deleteContactById = (id: string) => {
  const url = baseUrl + "/contacts/" + id;
  return fetch(url, {
    method: "delete",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

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

export const signup = (info: {
  phoneNumber: string | null;
  email: string | null;
  password: string;
  firstName: string;
  middleName: string | null;
  lastName: string;
  dateOfBirth: string;
}) => {
  const url = baseUrl + "/auth/signup";
  return fetch(url, {
    method: "POST",
    body: JSON.stringify(info),
    headers: {
      "Content-Type": "application/json",
    },
  });
};

export const addService = (service: {
  title: string;
  price: number;
  duration: string;
}) => {
  const url = baseUrl + "/services";
  return fetch(url, {
    method: "POST",
    body: JSON.stringify(service),
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

export const getAllServices = async () => {
  const url = baseUrl + "/services";
  const res = await fetch(url);
  return await res.json();
};

export const searchService = async (title: string) => {
  const url = baseUrl + "/services/search/title?title=" + title;
  const res = await fetch(url);
  return await res.json();
};

export const getService = async (id: string) => {
  const url = baseUrl + "/services/" + id;
  const res = await fetch(url);
  return await res.json();
};

export const updateServiceById = (
  id: string,
  service: {
    title: string;
    price: number;
    duration: string;
  }
) => {
  const url = baseUrl + "/services/" + id;
  return fetch(url, {
    method: "PUT",
    body: JSON.stringify(service),
    headers: {
      "Content-Type": "application/json",
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

export const deleteServiceById = (id: string) => {
  const url = baseUrl + "/services/" + id;
  return fetch(url, {
    method: "DELETE",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

export const getPdfPriceList = async () => {
  const url = baseUrl + "/services/priceList/pdf";
  const res = await fetch(url);
  return await res.blob();
};

export const getAvailableTime = async (serviceId: string) => {
  const url = baseUrl + "/orders/service/time/" + serviceId;
  const res = await fetch(url);
  return await res.json();
};

export const postOrder = (info: {
  serviceId: string;
  phoneNumber: string;
  email: string | null;
  carVin: string;
  carMake: string;
  carModel: string;
  carYear: number;
  startTime: string;
}) => {
  const url = baseUrl + "/orders";
  return fetch(url, {
    method: "POST",
    body: JSON.stringify(info),
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
  carVin: string;
  carMake: string;
  carModel: string;
  carYear: number;
  startTime: string;
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

export const getAllOrdersForSelf = async () => {
  const url = baseUrl + "/orders/self/all";
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

export const deleteOrderById = (id: string) => {
  const url = baseUrl + "/orders/" + id;
  return fetch(url, {
    method: "DELETE",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

export const deleteOrdersByIds = (ids: string[]) => {
  const url = baseUrl + "/orders?ids=" + ids.join(",");
  return fetch(url, {
    method: "DELETE",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
};

export const getReceiptByIds = async (ids: string[]) => {
  const url = baseUrl + "/orders/admin/receipt?ids=" + ids.join(",");
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.blob();
};

export const getReceiptByIdsForSelf = async (ids: string[]) => {
  const url = baseUrl + "/orders/self/receipt?ids=" + ids.join(",");
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.blob();
};

export const getServicesReport = async () => {
  const url = baseUrl + "/reports/services";
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};

export const getClientsReport = async () => {
  const url = baseUrl + "/reports/clients";
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};

export const getCarsReport = async () => {
  const url = baseUrl + "/reports/cars";
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};

export const getHoursReport = async () => {
  const url = baseUrl + "/reports/hours";
  const res = await fetch(url, {
    method: "GET",
    headers: {
      Authorization: "Bearer " + localStorage.getItem("accessToken") || "",
    },
  });
  return await res.json();
};
