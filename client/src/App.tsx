import Header from "./components/header";
import { Route, Routes } from "react-router-dom";
import Home from "./pages/home";
import Login from "./pages/login";
import Signup from "./pages/signup";
import BookService from "./pages/bookService";
import AdminOrders from "./pages/admin/orders";
import OrdersByService from "./pages/admin/orders/tableByServiceId";
import Contacts from "./pages/admin/contacts/table";
import Services from "./pages/admin/services/table";
import AdminPanel from "./pages/adminPanel";
import SqlEdit from "./pages/admin/sql";
import AdminAddService from "./pages/admin/services/add";
import AdminEditService from "./pages/admin/services/edit";
import AdminEditOrder from "./pages/admin/orders/edit";
import AdminEditContact from "./pages/admin/contacts/edit";
import EditContact from "./pages/admin/contacts/editSelf";
import AddCarSelf from "./pages/cars/add";
import EditCarSelf from "./pages/cars/edit";
import TableCarsSelf from "./pages/cars/table";
import AddCar from "./pages/admin/cars/add";
import EditCar from "./pages/admin/cars/edit";
import CarsTable from "./pages/admin/cars/table";

function App() {
  return (
    <>
      <Header />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/auth/login" element={<Login />} />
        <Route path="/auth/signup" element={<Signup />} />
        <Route path="/contacts/edit" element={<EditContact />} />
        <Route path="/admin" element={<AdminPanel />} />
        <Route path="/admin/sql" element={<SqlEdit />} />
        <Route path="/services/:id" element={<BookService />} />
        <Route path="/admin/orders" element={<AdminOrders />} />
        <Route path="/admin/orders/:id" element={<OrdersByService />} />
        <Route path="/admin/orders/edit/:id" element={<AdminEditOrder />} />
        <Route path="/admin/contacts" element={<Contacts />} />
        <Route path="/admin/contacts/edit/:id" element={<AdminEditContact />} />
        <Route path="/admin/services" element={<Services />} />
        <Route path="/admin/services/add" element={<AdminAddService />} />
        <Route path="/admin/services/edit/:id" element={<AdminEditService />} />
        <Route path="/cars/add" element={<AddCarSelf />} />
        <Route path="/cars/edit/:vin" element={<EditCarSelf />} />
        <Route path="/cars" element={<TableCarsSelf />} />
        <Route path="/admin/cars/add" element={<AddCar />} />
        <Route path="/admin/cars/edit/:vin" element={<EditCar />} />
        <Route path="/admin/cars" element={<CarsTable />} />
      </Routes>
    </>
  );
}

export default App;
