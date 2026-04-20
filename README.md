# iRent - iPhone Rental DApp

**iRent** - Decentralized iPhone Rental System with Smart Contract-Based CRUD Operations

---

## 📌 Project Description

iRent is a decentralized application (DApp) built on the Stellar blockchain using the Soroban SDK. This system enables users to manage iPhone rentals in a secure, transparent, and immutable way directly on-chain.

The application goes beyond basic CRUD functionality by implementing a real-world rental workflow, including device management, customer handling, rental transactions, and lifecycle tracking (Available → Rented → Returned).

All data is stored within the smart contract’s instance storage, ensuring persistence, integrity, and decentralization without relying on centralized databases.

---

## 🎯 Project Vision

Our vision is to modernize rental systems by leveraging blockchain technology:

- Decentralizing rental data management
- Ensuring transparency in rental transactions
- Providing tamper-proof records
- Enabling trustless interactions between users
- Creating scalable and secure decentralized business applications

---

## 🚀 Key Features

### 📱 1. iPhone Management
- Add new iPhone units
- Set rental price per day
- Track device status (Available, Rented, Maintenance)

### 👤 2. Customer Management
- Register customers
- Manage customer data on-chain

### 📄 3. Rental Transactions
- Rent iPhone devices
- Automatically calculate total rental price
- Store transaction details securely

### 🔄 4. Rental Lifecycle Management
- Track rental status (Ongoing, Returned)
- Automatically update iPhone availability upon return

### 🔍 5. Availability Filtering
- Retrieve only available iPhones
- Improve efficiency in rental selection

### 📊 6. Statistics Dashboard
- View number of available devices
- View number of rented devices

### 🔐 7. Transparency & Security
- All actions recorded on blockchain
- Immutable and verifiable data
- No unauthorized modifications

---

## 🧠 Smart Contract Functions

- `add_iphone(model, price_per_day)`  
- `get_iphones()`  
- `get_available_iphones()`  
- `add_customer(name)`  
- `rent_iphone(iphone_id, customer_id, days)`  
- `return_iphone(rental_id)`  
- `get_stats()`  

---

## 🏗️ Data Structures

### iPhone
- id  
- model  
- price_per_day  
- status  

### Customer
- id  
- name  

### Rental
- id  
- iphone_id  
- customer_id  
- days  
- total_price  
- status  

---

## 📍 Contract Details

- **Contract Name**: IRentContract  
- **Platform**: Stellar Soroban  
- **Language**: Rust  

---

## 🛠️ Technical Requirements

- Rust Programming Language  
- Soroban SDK  
- Stellar Blockchain Network  

---

## ⚙️ Getting Started

1. Deploy the smart contract to the Stellar Soroban network  
2. Interact using available functions:

- Add iPhone → `add_iphone()`  
- Add Customer → `add_customer()`  
- View iPhones → `get_iphones()`  
- Rent iPhone → `rent_iphone()`  
- Return iPhone → `return_iphone()`  
- Check Stats → `get_stats()`  

---

## 🔮 Future Scope

### Short-Term Enhancements
1. Late return penalty system  
2. Search and filtering features  
3. iPhone condition tracking  
4. Transaction history per customer  

### Medium-Term Development
5. Multi-user authentication (wallet address)  
6. Payment integration  
7. Notification system  
8. Rental extension feature  

### Long-Term Vision
9. NFT-based rental proof  
10. Cross-chain integration  
11. Decentralized frontend hosting (IPFS)  
12. DAO-based governance  

---

## 📊 Use Case

- Mobile device rental businesses  
- Gadget leasing platforms  
- Temporary device usage services  
- Digital asset rental systems  

---

## 🧾 Conclusion

iRent is not just a CRUD application—it is a decentralized rental management system that simulates real-world business processes using blockchain technology. With features like lifecycle tracking, automated pricing, and transparent transactions, iRent provides a solid foundation for scalable decentralized applications.

---

## ✨ Tagline

**iRent**  
_"Smart Rentals. Transparent. Decentralized."_

Contract Id
- CDLUNEVK72WGNX6EJCAYV7THKWQCP2UKLO5SB5XXKYA2FLAW3KCIP2YF


<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/afcd2de9-d93c-40ba-be01-8b5aa2af0611" />
