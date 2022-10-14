type Contact = {
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
  contact: Contact;
  service: Service;
  startTime: string;
  carMake: string;
  carModel: string;
  carYear: number;
};
