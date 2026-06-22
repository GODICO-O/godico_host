#!/bin/bash

# Warna untuk estetika terminal
HIJAU='\033[0;32m'
BIRU='\033[0;34m'
KUNING='\033[1;33m'
MERAH='\033[0;31m'
NORMAL='\033[0m'

echo -e "${BIRU}====================================================${NORMAL}"
echo -e "${HIJAU}   🗿 GODICO AUTOMATED PUSH, WATCH & INSTALLER 🗿   ${NORMAL}"
echo -e "${BIRU}====================================================${NORMAL}"

# 1. OTOMATIS GAMPUR PUSH KE GITHUB
echo -e "\n${KUNING}[1/5] Mengunci kode dan mengirim ke awan GitHub...${NORMAL}"
git add .
git commit -m "Auto-build: Sinkronisasi ikon baru dan optimalisasi skrip"
git push origin main

if [ $? -ne 0 ]; then
    echo -e "${MERAH}❌ Gagal melakukan push! Periksa koneksi atau repositori lu.${NORMAL}"
    exit 1
fi

# JEDA 5 DETIK SESUAI INSTRUKSI STRATEGIS KOMANDAN
echo -e "\n${KUNING}⏳ Memberikan jeda 5 detik agar server GitHub bangun...${NORMAL}"
sleep 5

# 2. OTOMATIS MEMANTAU BUILD (GH RUN WATCH)
echo -e "\n${KUNING}[2/5] Menghubungi robot GitHub Actions. Memulai pengawasan...${NORMAL}"
gh run watch

# 3. OTOMATIS DOWNLOAD ARTIFACT DARI RUN TERBARU
echo -e "\n${KUNING}[3/5] Build Sukses! Mengunduh biner APK terbaru dari awan...${NORMAL}"
rm -rf tmp_godico_apk
mkdir -p tmp_godico_apk

# Mengambil ID run terakhir dari workflow android.yml
RUN_ID=$(gh run list --workflow="android.yml" --limit 1 --json databaseId -q '.[0].databaseId')
gh run download $RUN_ID -n GODICO-DevHub-APK --dir tmp_godico_apk

# 4. MEMASTIKAN FILE APK BERHASIL DIEKSTRAK
if [ ! -f tmp_godico_apk/app-debug.apk ]; then
    echo -e "${MERAH}❌ Gagal mengunduh atau mengekstrak APK dari GitHub!${NORMAL}"
    exit 1
fi

echo -e "${HIJAU}✅ APK Berhasil diunduh dan diekstrak!${NORMAL}"

# 5. MEMICU POP-UP INSTALASI DAN UNINSTALL DI ANDROID
echo -e "\n${KUNING}[4/5] Mengirim instruksi eksekusi ke Android...${NORMAL}"

# Salin APK langsung ke penyimpanan download (tanpa trigger setup storage lagi)
cp tmp_godico_apk/app-debug.apk /sdcard/Download/godico-devhub.apk

echo -e "${BIRU}ℹ️  Membuka halaman Uninstall Versi Lama (jika terpasang)...${NORMAL}"
am start -a android.intent.action.DELETE -d "package:com.godico.devhub" --user 0 > /dev/null 2>&1

echo -e "${HIJAU}👉 Silakan klik 'OKE' jika muncul pop-up uninstall di layar HP lu.${NORMAL}"
echo -e "${KUNING}Menunggu 5 detik sebelum memunculkan pop-up instalasi baru...${NORMAL}"
sleep 5

echo -e "\n${KUNING}[5/5] MEMUNCULKAN POP-UP INSTALASI APK BARU...${NORMAL}"
am start -a android.intent.action.VIEW \
    -d "file:///sdcard/Download/godico-devhub.apk" \
    -t "application/vnd.android.package-archive" --user 0 > /dev/null 2>&1

# Bersihkan folder sampah di Termux
rm -rf tmp_godico_apk

echo -e "\n${HIJAU}====================================================${NORMAL}"
echo -e "${HIJAU}✅ SCRIPT SELESAI! Pelototi layar HP Oppo lu sekarang!${NORMAL}"
echo -e "${BIRU}====================================================${NORMAL}"
