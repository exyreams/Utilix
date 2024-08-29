## Utilix: Command-Line Toolkit

![enter image description here](https://raw.githubusercontent.com/exyreams/collections/main/utilx_tools.gif?token=GHSAT0AAAAAACTM6Q5SPZXRISYE6WOQF5IQZWQHQWQ)

Utilix is a comprehensive command-line toolkit designed to simplify common data manipulation tasks directly within your terminal. With a variety of built-in utilities, Utilix empowers you to encode, convert, generate, and hash data efficiently without needing a graphical interface.

**Why Choose Utilix?**

* **All-in-One Utility:** Combine several helpful tools in a single, convenient application.
* **Terminal-Based Power:** Manipulate data directly in your terminal, perfect for scripting and automation.
* **User-Friendly Interface:** Utilix features a text-based menu with intuitive controls, providing a clear and straightforward experience. 
* **Comprehensive Features:**  Utilix offers a wide range of functionalities including:

### **Getting Started : Building Your Coding Sanctuary**

**1. Install Rust:**  **Prerequisites:**  You'll need Rust and Cargo installed on your system. 
- Check Rust's version:  `rustc --version`

- **Install Rust using official installer:**  
	Link:  [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)  

**2. Clone Utilix:**
```
https://github.com/exyreams/Utilix.git
cd Utilix
```

**3. Build utilix:**

```
cargo build
```

**4. Run utilix:**

```
target/debug/utilix
	or
cargo run
```

### **Tools Guide:**

#### **Base64 Encoder/Decoder:**

![base64_tool](https://github.com/user-attachments/assets/0711cf1c-31ab-459d-88db-77af91689ba5)

Encode data in Base64 format for storage, transmission, or embedding in URLs. Decode Base64-encoded strings to recover original data.
- **Guide**:
	- **`Esc`**     : Quits Program
	- **`Tab`**     : Switch to Next Tool
	- **`Alt + e`** : Encode Input
	- **`Alt + d`** : Decode Input
	- **`Alt + x`** : Export Input, Encode, Decode in **`.txt`** file in **`export/base64.txt`** directory. 

#### **Color Code Converter:**

![color_code_converter_tool](https://github.com/user-attachments/assets/fb291666-18e8-4028-a54a-de817e87e765)

Convert color codes between **`RGB`**, **`HEX`**, **`CMYK`**, and **`HSL`** formats, making it easy to work with different color representation systems. 
- **Guide**:
	- **`Esc`**        : Quits Program
	- **`Tab`**        : Switch to Next Tool
	- **`Alt + x`**    : Export input & all the color codes in  **`.txt`** file in **`export/color_codes.txt`** directory. 
- **Input Formats with Examples**:
	- **CMYK Color Code**  : **`0, 26, 99, 1 or 0%, 26%, 99%, 1%`**
	- **HEX Color Code**   : **`#fcba03` or `#FCBA03`** 
	- **HSL Color Code**   : **`44, 98, 50 or 44°, 98%, 50%  `** 
	- **RGB Color Code**   : **`252, 186, 3`**

#### **Date Converter:**

![date_converter_tool](https://github.com/user-attachments/assets/1c1f72af-8a78-46fe-9ccb-d2af7f1f51ab)

Convert dates between multiple formats, including **`RFC 3339`**, **`RFC 2822`**, **`ISO 8601`**, **`Unix timestamps`**, **`human-readable formats`**, and **`short date`** representations. 
- **Guide**:
	- **`Esc`**  : Quits Program
	- **`Tab`**  : Switch to Next Tool
	- **Export feature isn't available** for date converter tool, users can directly use the terminal to copy the dates.
- **Supported Formats:**
	- **`YYYY-MM-DD H:M:S`**
	- **`YYYY-MM-DDTH:M:S:z`**
	- **`YYYY-MM-DD`**
	- **`DD/MM/YYYY H:M:S`**
	- **`DD/MM/YYYY`**
- **Examples:**
	- **RFC 3339**        :  `2024-05-22T13:00:00Z`
	- **RFC 2822**        :  `Tue, 22 May 2022 13:00:00 +0100`
	- **ISO 8601**        :  `2024-05-22T13:00:00+01:00 or 20240522T130000+0100`
	- **Unix Timestamp**  : `1716382800`
	- **Human Readable**  : `Tuesday, March 1, 2022, 1:00:00 PM`
	- **Time Only**       : `1:00:00 PM or 13:00:00`

#### **Hash Generator:**

![hash_generator_tool](https://github.com/user-attachments/assets/9ba6f51c-c888-4a87-85d1-179341fa6b1c)

Generate secure hashes using **`SHA-1`**, **`SHA-256`**, **`SHA-384`**, and **`SHA-512`** algorithms, ideal for data integrity verification or secure storage from the given input simultaneously.
- **Guide**:
	- **`Esc`**          : Quits Program
	- **`Tab`**          : Switch to Next Tool
	- **`Alt + x`**      : Export input and all hashed output in **`.txt`** file in **`export/hash.txt`** directory. 

#### **Number Base Converter:**

![number_base_converter_tool](https://github.com/user-attachments/assets/825825df-0e03-426f-98a4-0470aa28bacc)

Convert numbers between binary, decimal, and hexadecimal representations, simplifying calculations and data processing. It can convert to:
	- Binary ==> Decimal & Hexadecimal
	- Decimal ==> Binary & Hexadecimal
	- Hexadecimal ==> Binary & Decimal
- **Guide**:
	- **`Esc`**         : Quits Program
	- **`Tab`**         : Switch to Next Tool
	- **`Alt + x`**     : Export input and all conversions in  **`.txt`** file in **`export/number_conversion.txt`** directory. 

#### **Password Generator:**

![password_generator_tool](https://github.com/user-attachments/assets/c96c1b3d-fe15-40f1-97ab-fd146e30478b)

Generate strong and secure passwords with customization options, allowing for length control, inclusion of upper/lowercase letters, numbers, symbols, and character restriction settings.
- **Settings Details:**
	- **`Length:`**  Total number of characters in the generated password.
	- **`Quantity:`**  Specifies the number of passwords to generate. 
	- **`Uppercase:`**  Indicates whether uppercase letters (A-Z) should be included in the password. 
	- **`Lowercase:`**  Determines whether lowercase letters (a-z) should be part of the password. 
	- **`Numbers:`**  Specifies whether numeric digits (0-9) should be included. 
	- **`Symbols:`**  Defines whether special characters (e.g., !, @, #, $, %, &, *) should be included. 
	- **`Similar characters:`**  Indicates whether similar characters (e.g., I, l, 1, O, 0) should be avoided. 
	- **`Duplicate characters:`**  Determines whether repeating characters (e.g. aa, bb, pp, 11 ) are allowed in the password. 
	- **`Sequential Characters:`**  Specifies whether sequences of characters (e.g., abc, efg, 123, 456) should be avoided. 
- **Guide**:
	- **`Esc`**  : Quits Program
	- **`Tab`**  : Switch to Next Tool
	- **`g`**    : Generate password
	- **`c`**    : Clear generated password
	- **`x`**    : Export generate password **`.txt`** file in **`export/password.txt`** directory. 
	- **`i`**    : Increase password length
	- **`d`**    : Decrease password length
	- **`m`**    : Generate multiple password
		<span style="color: yellow; font-weight: bold;">Warning:</span> Multi generating doesn't work as of now.
	- **`k`**    : Increase password quantity
	- **`j`**    : Decrease password quantity
	- **`u`**    : Toggle to include uppercase characters
	- **`l`**    : Toggle to include lowercase characters
	- **`n`**    : Toggle to include numbers
	- **`s`**    : Toggle to include symbols
	- **`z`**    : Toggle to include similar characters
	- **`q`**    : Toggle to include duplicate characters
	- **`v`**    : Toggle to include sequential characters

#### **QR Code Generator:**

![qr_generator_tool](https://github.com/user-attachments/assets/a9cae818-dfed-4248-9385-6d567c17614d)

Generate QR codes from URLs, text, and more.  utilix handles QR code creation and conveniently export the image in **`.png`** format, for more you can use QR help menu. it will automatically start generating the QR Code once you input values in the input field.
- **Guide**:
	- **`Esc`**         : Quits Program
	- **`Tab`**         : Switch to Next Tool
	- **`Alt + x`**     : Export generated QR Code in **`.png`** image in **`export/{First10inputcharacters}.png`** directory. 
	  
	<span style="color: Red; font-weight: bold;">Note: </span> The generated qr takes first 10 characters from the input field and use it for your qr code image name.

#### **UUID Generator (v4 & v7):**  

![uuid_tool](https://github.com/user-attachments/assets/5033eed2-4cff-4aa6-81e1-495a6ccdb08b)

Generate version 4 (randomly generated) and version 7 (time-based) UUIDs, providing unique identifiers for various applications usecase.
- **Settings:**
	- **`Number of UUID:`**  Amount of UUID to generate, default it 0. it can be increased as per your need. 
- **Guide**:
	- **`Esc`**  : Quits Program
	- **`Tab`**  : Switch to Next Tool
	- **`s`**    : Generate single V4 UUID
	- **`w`**    : Generate single V7 UUID
	- **`m`**    : Generate multiple V4 UUID
	- **`e`**    : Generate multiple V7 UUID
	- **`i`**    : Increase number of UUIDs to generate
	- **`d`**    : Increase number of UUIDs to generate
	- **`x`**    : Export generated UUID in  **`.txt`** file in **`export/uuid.txt`** directory. 

####  **File Export:**  
Export your results conveniently to text files in a designated **`export`** directory on the root of the program. **`Alt + x`** or **`x`** key can be used to export the files.

### **Utilix Demo**:
[utilix_all_tools_demo.webm](https://github.com/user-attachments/assets/7b9bc66f-a3c3-4e13-bee9-a13549d487dd)
