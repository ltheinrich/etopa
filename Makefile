OUTPUT=${PWD}/target/build
EXTRA=extra
BUILDER=cross

# api
API_FILE=etopa
API_TARGET=x86_64-unknown-linux-musl
API_STRIP=strip
RPM_FILE=etopa.rpm
DEB_FILE=etopa.deb

# android
NDK_ARM64=$(HOME)/.android/ndk/arm64/bin
NDK_ARM=$(HOME)/.android/ndk/arm/bin
export PATH := ${NDK_ARM64}:${NDK_ARM}:$(PATH)
ANDROID_BT=~/.android/sdk/build-tools/30.0.2
JNI_LIBS=etopan-app/app/src/main/jniLibs
BUNDLETOOL=~/.bundletool-all.jar
AAB_FILE=${EXTRA}/etopa.aab
APK_FILE=etopa.apk
S2APK_FILE=${EXTRA}/etopa-fdroid.apk
UAPK_FILE=${EXTRA}/etopa-unsigned.apk
MAPPING=${EXTRA}/mapping.txt
DEBUG_SYMBOLS=${EXTRA}/native-debug-symbols.zip
KEYSTORE=~/.etopa.jks
KS_PASS=$(shell cat ~/.etopa.jks.pass)
KS_ALIAS=etopa

# web
WEB_FILE=etopa.tar.xz
WASM_PACK=wasm-pack
GOMINIFY=minify-v2.8.0 # use v2.8.0 (-> v2.9.0 breaks code)
TEMP_EWM=/tmp/etopa_ewm

.PHONY: build signed api web android sign rpm deb

build: rmtarget notice api web android rpm deb
	\cp NOTICE.txt ${OUTPUT}/NOTICE.txt

full: build sign

api:
	mkdir -p ${OUTPUT} && mkdir -p ${OUTPUT}/${EXTRA}
	rm -f ${OUTPUT}/${API_FILE}
	${BUILDER} build -p etopai --release --target ${API_TARGET}
	${API_STRIP} target/${API_TARGET}/release/etopai
	cp target/${API_TARGET}/release/etopai ${OUTPUT}/${API_FILE}

web:
	mkdir -p ${OUTPUT} && mkdir -p ${OUTPUT}/${EXTRA}
	rm -f ${OUTPUT}/${WEB_FILE} && rm -rf ${TEMP_EWM}
	${WASM_PACK} build --release --no-typescript -t web -d ../etopaw-app/pkg etopaw
	cp -r etopaw-app ${TEMP_EWM}
	${GOMINIFY} -r -o ${TEMP_EWM}/ etopaw-app/
	\cp etopaw-app/config.js ${TEMP_EWM}/config.js
	cp NOTICE.txt ${TEMP_EWM}/NOTICE.txt
	(cd ${TEMP_EWM} && tar cfJ ${OUTPUT}/etopa.tar.xz *)
	rm -rf ${TEMP_EWM}

android:
	mkdir -p ${OUTPUT} && mkdir -p ${OUTPUT}/${EXTRA}
	rm -f ${OUTPUT}/${AAB_FILE} && rm -f ${OUTPUT}/${APK_FILE} && rm -f ${OUTPUT}/${S2APK_FILE} && rm -f ${OUTPUT}/${UAPK_FILE}
	${BUILDER} build -p etopan --release --target aarch64-linux-android
	${BUILDER} build -p etopan --release --target armv7-linux-androideabi
	rm -rf ${JNI_LIBS} && mkdir -p ${JNI_LIBS}/arm64-v8a && mkdir -p ${JNI_LIBS}/armeabi-v7a
	cp target/aarch64-linux-android/release/libetopan.so ${JNI_LIBS}/arm64-v8a/libetopan.so
	cp target/armv7-linux-androideabi/release/libetopan.so ${JNI_LIBS}/armeabi-v7a/libetopan.so
	\cp NOTICE.txt etopan-app/app/src/main/assets/NOTICE.txt
	(cd etopan-app && ./gradlew clean && ./gradlew :app:bundleRelease && ./gradlew assembleRelease)
	cp etopan-app/app/build/outputs/apk/release/app-release-unsigned.apk ${OUTPUT}/${UAPK_FILE}

sign:
	java -jar ${BUNDLETOOL} build-bundle --modules=etopan-app/app/build/intermediates/module_bundle/release/base.zip --output=${OUTPUT}/${AAB_FILE}
	jarsigner -keystore ${KEYSTORE} -storepass ${KS_PASS} -sigalg SHA256withRSA -digest-alg SHA-256 ${OUTPUT}/${AAB_FILE} etopa
	# already aligned # ${ANDROID_BT}/zipalign -v -p 4 etopan-app/app/build/outputs/apk/release/app-release-unsigned.apk etopan-app/app/build/outputs/apk/release/app-release-unsigned-aligned.apk # change next line's file to ..unsigned-aligned.apk
	${ANDROID_BT}/apksigner sign --v4-signing-enabled false --v3-signing-enabled true --ks ${KEYSTORE} --ks-key-alias ${KS_ALIAS} --ks-pass pass:${KS_PASS} --out ${OUTPUT}/${APK_FILE} etopan-app/app/build/outputs/apk/release/app-release-unsigned.apk
	${ANDROID_BT}/apksigner sign --v4-signing-enabled false --v3-signing-enabled false --v2-signing-enabled true --ks ${KEYSTORE} --ks-key-alias ${KS_ALIAS} --ks-pass pass:${KS_PASS} --out ${OUTPUT}/${S2APK_FILE} etopan-app/app/build/outputs/apk/release/app-release-unsigned.apk
	cp etopan-app/app/build/outputs/mapping/release/mapping.txt ${OUTPUT}/${MAPPING}
	cp etopan-app/app/build/outputs/native-debug-symbols/release/native-debug-symbols.zip ${OUTPUT}/${DEBUG_SYMBOLS}

rpm:
	mkdir -p ${OUTPUT} && mkdir -p ${OUTPUT}/${EXTRA}
	rm -f ${OUTPUT}/${RPM_FILE}
	(cd etopai && cargo rpm build --no-cargo-build --target ${API_TARGET} -v)
	cp target/${API_TARGET}/release/rpmbuild/RPMS/*/etopa-*.rpm ${OUTPUT}/${RPM_FILE}

deb:
	mkdir -p ${OUTPUT} && mkdir -p ${OUTPUT}/${EXTRA} && rm -f ${OUTPUT}/${DEB_FILE}
	cargo deb -p etopai --no-build --target ${API_TARGET}
	cp target/${API_TARGET}/debian/etopa_*.deb ${OUTPUT}/${DEB_FILE}

notice:
	head -841 NOTICE.txt > NOTICE.txt.tmp && mv NOTICE.txt{.tmp,}
	cargo-license -t | sed "s/ring\t\tLICENSE/ring\t\tring's license/g" | sed "s/webpki\t\tLICENSE/ring\t\tISC AND BSD-3-Clause/g" >> NOTICE.txt

rmtarget:
	rm -rf target/build
