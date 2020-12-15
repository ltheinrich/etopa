TARGET_OUTPUT_DIR?=${PWD}/target/build
EXTRA_DIR?=extra
RUST_BUILDER?=cargo
NOTICE_FILE?=NOTICE.txt

# api
API_FILE_NAME?=etopa
API_NATIVE_FILE_NAME?=${EXTRA_DIR}/etopa-native
API_TARGET_TRIPLE?=x86_64-unknown-linux-musl
API_STRIP?=strip
API_RPM_FILE?=etopa.rpm
API_DEB_FILE?=etopa.deb
CARGO_RPM?=cargo rpm
CARGO_DEB?=cargo deb
CARGO_LICENSE?=cargo-license

# android
NDK_BIN_PATH?=$(HOME)/.android/sdk/ndk/21.3.6528147
export PATH := ${NDK_BIN_PATH}/toolchains/llvm/prebuilt/linux-x86_64/bin:$(PATH)
ANDROID_BT_PATH?=~/.android/sdk/build-tools/30.0.3
JNI_LIBS_PATH?=etopan-app/app/src/main/jniLibs
BUNDLETOOL_JAR?=~/.bundletool-all.jar
ANDROID_AAB_FILE?=${EXTRA_DIR}/etopa.aab
ANDROID_APK_FILE?=etopa.apk
ANDROID_S2APK_FILE?=${EXTRA_DIR}/etopa-fdroid.apk
ANDROID_UAPK_FILE?=${EXTRA_DIR}/etopa-unsigned.apk
ANDROID_MAPPING?=${EXTRA_DIR}/mapping.txt
ANDROID_DEBUG_SYMBOLS?=${EXTRA_DIR}/native-debug-symbols.zip
ANDROID_KEYSTORE?=~/.etopa.jks
JKS_PASS_FILE?=$(shell cat ~/.etopa.jks.pass)
JKS_ALIAS?=etopa
JAVA_EXEC=java
JARSIGNER_EXEC=jarsigner
APKSIGNER_EXEC=${ANDROID_BT_PATH}/apksigner
ZIPALIGN_EXEC=${ANDROID_BT_PATH}/zipalign

# web
WEB_FILE_NAME?=etopa.tar.xz
WASM_PACK_EXEC?=wasm-pack
GOMINIFY_EXEC?=minify-v2.8.0 # use v2.8.0 (-> v2.9.0 breaks code)
TEMP_EWM?=/tmp/etopa_ewm

.PHONY: build signed api web android sign rpm deb docker

build: rmtarget notice api web android rpm deb
	\cp ${NOTICE_FILE} ${TARGET_OUTPUT_DIR}/NOTICE.txt

full: build sign

api:
	mkdir -p ${TARGET_OUTPUT_DIR} && mkdir -p ${TARGET_OUTPUT_DIR}/${EXTRA_DIR}
	rm -f ${TARGET_OUTPUT_DIR}/${API_FILE_NAME}
	${RUST_BUILDER} rustc -p etopai --release --target ${API_TARGET_TRIPLE} -- -C target-cpu=native
	${API_STRIP} target/${API_TARGET_TRIPLE}/release/etopai
	mv target/${API_TARGET_TRIPLE}/release/etopai ${TARGET_OUTPUT_DIR}/${API_NATIVE_FILE_NAME}
	${RUST_BUILDER} build -p etopai --release --target ${API_TARGET_TRIPLE}
	${API_STRIP} target/${API_TARGET_TRIPLE}/release/etopai
	cp target/${API_TARGET_TRIPLE}/release/etopai ${TARGET_OUTPUT_DIR}/${API_FILE_NAME}

web:
	mkdir -p ${TARGET_OUTPUT_DIR} && mkdir -p ${TARGET_OUTPUT_DIR}/${EXTRA_DIR}
	rm -f ${TARGET_OUTPUT_DIR}/${WEB_FILE_NAME} && rm -rf ${TEMP_EWM}
	${WASM_PACK_EXEC} build --release --no-typescript -t web -d ../etopaw-app/pkg etopaw
	cp -r etopaw-app ${TEMP_EWM}
	${GOMINIFY_EXEC} -r -o ${TEMP_EWM}/ etopaw-app/
	\cp etopaw-app/config.js ${TEMP_EWM}/config.js
	cp ${NOTICE_FILE} ${TEMP_EWM}/NOTICE.txt
	(cd ${TEMP_EWM} && tar cfJ ${TARGET_OUTPUT_DIR}/etopa.tar.xz *)
	rm -rf ${TEMP_EWM}

android: export CC_aarch64_linux-android = aarch64-linux-android30-clang
android: export CC_armv7_linux-androideabi = armv7a-linux-androideabi30-clang
android:
	mkdir -p ${TARGET_OUTPUT_DIR} && mkdir -p ${TARGET_OUTPUT_DIR}/${EXTRA_DIR}
	rm -f ${TARGET_OUTPUT_DIR}/${ANDROID_AAB_FILE} && rm -f ${TARGET_OUTPUT_DIR}/${ANDROID_APK_FILE} && \
	  rm -f ${TARGET_OUTPUT_DIR}/${ANDROID_S2APK_FILE} && rm -f ${TARGET_OUTPUT_DIR}/${ANDROID_UAPK_FILE}
	${RUST_BUILDER} build -p etopan --release --target aarch64-linux-android
	${RUST_BUILDER} build -p etopan --release --target armv7-linux-androideabi
	rm -rf ${JNI_LIBS_PATH} && mkdir -p ${JNI_LIBS_PATH}/arm64-v8a && mkdir -p ${JNI_LIBS_PATH}/armeabi-v7a
	cp target/aarch64-linux-android/release/libetopan.so ${JNI_LIBS_PATH}/arm64-v8a/libetopan.so
	cp target/armv7-linux-androideabi/release/libetopan.so ${JNI_LIBS_PATH}/armeabi-v7a/libetopan.so
	mkdir -p etopan-app/app/src/main/assets && \cp ${NOTICE_FILE} etopan-app/app/src/main/assets/NOTICE.txt
	(cd etopan-app && ./gradlew :app:bundleRelease && ./gradlew assembleRelease)
	cp etopan-app/app/build/outputs/apk/release/app-release-unsigned.apk ${TARGET_OUTPUT_DIR}/${ANDROID_UAPK_FILE}

sign:
	${JAVA_EXEC} -jar ${BUNDLETOOL_JAR} build-bundle \
	  --modules=etopan-app/app/build/intermediates/module_bundle/release/base.zip --output=${TARGET_OUTPUT_DIR}/${ANDROID_AAB_FILE}
	${JARSIGNER_EXEC} -keystore ${ANDROID_KEYSTORE} -storepass ${JKS_PASS_FILE} -sigalg SHA256withRSA \
	  -digest-alg SHA-256 ${TARGET_OUTPUT_DIR}/${ANDROID_AAB_FILE} etopa
	#${ZIPALIGN_EXEC} -v 4 etopan-app/app/build/outputs/apk/release/app-release-unsigned.apk etopan-app/app/build/outputs/apk/release/app-release-unsigned-aligned.apk # change next line
	${APKSIGNER_EXEC} sign --v4-signing-enabled false --v3-signing-enabled true --ks ${ANDROID_KEYSTORE} --ks-key-alias ${JKS_ALIAS} \
	  --ks-pass pass:${JKS_PASS_FILE} --out ${TARGET_OUTPUT_DIR}/${ANDROID_APK_FILE} etopan-app/app/build/outputs/apk/release/app-release-unsigned.apk
	${APKSIGNER_EXEC} sign --v4-signing-enabled false --v3-signing-enabled false --v2-signing-enabled true --ks ${ANDROID_KEYSTORE} \
	  --ks-key-alias ${JKS_ALIAS} --ks-pass pass:${JKS_PASS_FILE} --out ${TARGET_OUTPUT_DIR}/${ANDROID_S2APK_FILE} \
	  etopan-app/app/build/outputs/apk/release/app-release-unsigned.apk
	cp etopan-app/app/build/outputs/mapping/release/mapping.txt ${TARGET_OUTPUT_DIR}/${ANDROID_MAPPING}
	cp etopan-app/app/build/outputs/native-debug-symbols/release/native-debug-symbols.zip ${TARGET_OUTPUT_DIR}/${ANDROID_DEBUG_SYMBOLS}

rpm:
	mkdir -p ${TARGET_OUTPUT_DIR} && mkdir -p ${TARGET_OUTPUT_DIR}/${EXTRA_DIR}
	rm -f ${TARGET_OUTPUT_DIR}/${API_RPM_FILE}
	(cd etopai && ${CARGO_RPM} build --no-cargo-build --target ${API_TARGET_TRIPLE} -v)
	cp target/${API_TARGET_TRIPLE}/release/rpmbuild/RPMS/*/etopa-*.rpm ${TARGET_OUTPUT_DIR}/${API_RPM_FILE}

deb:
	mkdir -p ${TARGET_OUTPUT_DIR} && mkdir -p ${TARGET_OUTPUT_DIR}/${EXTRA_DIR} && rm -f ${TARGET_OUTPUT_DIR}/${API_DEB_FILE}
	${CARGO_DEB} -p etopai --no-build --target ${API_TARGET_TRIPLE} -v
	cp target/${API_TARGET_TRIPLE}/debian/etopa_*.deb ${TARGET_OUTPUT_DIR}/${API_DEB_FILE}

notice:
	head -841 ${NOTICE_FILE} > ${NOTICE_FILE}.tmp && mv ${NOTICE_FILE}.tmp ${NOTICE_FILE}
	${CARGO_LICENSE} -t | sed "s/ring\t\tLICENSE/ring\t\tring's license/g" | sed "s/webpki\t\tLICENSE/ring\t\tISC AND BSD-3-Clause/g" >> ${NOTICE_FILE}

rmtarget:
	rm -rf ${TARGET_OUTPUT_DIR}

docker:
	docker build -t etopa-builder:v1 .
