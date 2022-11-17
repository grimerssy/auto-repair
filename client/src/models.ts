export type Car = {
  vin: string;
  contact: Contact;
  make: string;
  model: string;
  year: number;
};

export type Contact = {
  id: string;
  phoneNumber: string;
  email: string | null;
};

export type Service = {
  id: string;
  title: string;
  price: number;
  duration: string;
};

export type Order = {
  id: string;
  service: Service;
  worker: Worker;
  car: Car;
  startTime: string;
};

export type User = {
  contact: Contact;
  passwordHash: string;
  role: string;
  firstName: string;
  middleName: string | null;
  lastName: string;
  dateOfBirth: string;
  registeredAt: string;
};

export type Worker = {
  id: string;
  firstName: string;
  middleName: string | null;
  lastName: string;
  dateOfBirth: string;
  startTime: string;
  endTime: string;
};
