## ⚡ C-Template Installer – The Ultimate Rust CLI Tool for Bootstrapping C Projects 

### **What is this?**
Ever found yourself **wasting time** setting up the same **C project structure** again and again? Annoyed by manually downloading ZIPs from GitHub, extracting them, and renaming folders? 😩  

Well, **no more!** Meet **C-Template Installer** – a **blazing-fast Rust CLI tool** that does all of that **in seconds**. 🔥  

- **📥 Downloads** a C project template from GitHub  
- **📦 Extracts** it automatically  
- **🚀 Renames** it to your project name  
- **🧹 Cleans up** unnecessary files  

All with a **single command**. 🤯  

---

## 🎯 **Features**
✅ **One-command setup** – Just run and start coding 🚀  
✅ **Superfast execution** – Thanks to Rust’s raw speed ⚡  
✅ **No manual work** – No need to visit GitHub, download, unzip, rename, etc. 🛠️  
✅ **Works on Linux, Mac, and Windows** – Just needs `curl` & `tar` 🏗️  
✅ **Lightweight & minimal** – Zero unnecessary dependencies  

---

## 🛠 **Installation**
### **📌 Prerequisites**
Make sure you have:  
✅ Rust installed → [Install Rust](https://www.rust-lang.org/tools/install)  
✅ `curl` installed (for downloading)  
✅ `tar` installed (for extracting)  

#### **1️⃣ Clone this repository:**
```sh
git clone https://github.com/mmycin/C-Template-Installer.git
cd C-Template-Installer
```

#### **2️⃣ Build the CLI:**
```sh
cargo build --release
```
Your compiled binary will be in `target/release/c-template-installer`. 🎉  

#### **3️⃣ (Optional) Install globally:**
```sh
cp target/release/c-template-installer /usr/local/bin/c-template
```
Now, you can use it from anywhere! 🌎  

---

## 🚀 **Usage**
### **The Magic Command:**
```sh
c-template my-awesome-project
```
This will:  
1️⃣ Download the C template from GitHub  
2️⃣ Extract it into `my-awesome-project/`  
3️⃣ Delete unnecessary ZIP files  
4️⃣ Leave you ready to **start coding immediately**! 🚀  

### **Full Example**
```sh
c-template hello-world
```
**Output:**  
```
📥 Downloading repository...
✅ Download complete.
📦 Extracting repository...
✅ Extraction complete.
🧹 Cleaning up...
✅ Process completed successfully!
🎉 Your project is ready in 'hello-world/'!
```

Now, just:
```sh
cd hello-world
code .
```
And **BOOM**, you’re coding in seconds! 💥  

---

## 🛠 **How It Works (Under the Hood)**
This CLI tool performs these steps **automatically**:  

1️⃣ **Uses `reqwest`** to download a ZIP from GitHub  
2️⃣ **Saves it as `repo.zip`**  
3️⃣ **Extracts it with `zip` library**  
4️⃣ **Renames the extracted folder**  
5️⃣ **Deletes unnecessary files**  

⚡ **Everything happens in a few milliseconds!** ⚡  

---

## 🏆 **Why Use This?**
- **You’re tired** of manually setting up C projects 😴  
- **You want to start coding immediately** ⏩  
- **You love automation** and hate repetitive tasks 🤖  
- **You appreciate Rust’s speed and efficiency** 🚀  

---

## 🔥 **Contributing**
Found a bug? Want to add features? Feel free to contribute!  

1️⃣ Fork the repo 🍴  
2️⃣ Make your changes 🛠️  
3️⃣ Open a Pull Request 🔃  

---

## 📜 **License**
This project is open-source under the **MIT License**. Feel free to use, modify, and share! 💙  

---

## 🎯 **Final Thoughts**
If you’re working with C **(or just love cool automation tools)**, this CLI will **save you time** and make your life easier. Give it a try, and you’ll never set up a C project manually again! 🚀  

🚀 **Happy Coding!** 💻🔥  

---
