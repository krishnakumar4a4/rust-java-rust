extern crate jni_sys;
extern crate rucaja;
extern crate jni;

use jni::JNIEnv as jni_JNIEnv;
use jni::sys::jbyteArray as jni_jbyteArray;

use rucaja::{Jvm, JvmAttachment, JvmClass, JvmMethod};
use jni_sys::{_jobject, JNINativeInterface_, JNINativeMethod, jclass, jint, JNIEnv, jbyteArray};
use std::ffi::{CString, c_void};
use std::ptr::null;

fn main() {
    // JVM options.
    let jvm_options = [
        //"-Djava.security.debug=all",
        //"-verbose:gc",
        //"-verbose:jni",
        //"-Xcheck:jni",
    ];

    // Instantiate the embedded JVM.
    let jvm = Jvm::new(&jvm_options);

    // Attach the current native thread to the JVM.
    let jvm_attachment = JvmAttachment::new(jvm.jvm());

    // Get the Java class `Test` from `Test.class`.
    let class = JvmClass::get_class(&jvm_attachment, "RustJavaRust").expect("Rust: Could not find JVM class");

    // jni env
    let env = jvm_attachment.jni_environment();

    register_native_funcs_with_java(env, &class);

    call_java_constructor(&jvm_attachment, &class);
}

fn register_native_funcs_with_java<'r>(env: *mut *const JNINativeInterface_, class: &JvmClass) {
    // Native function name:
    let method_name_call_from_java = CString::new("call_from_java").ok().unwrap().into_raw();
    let method_name_call_from_java_with_int = CString::new("call_from_java_with_int").ok().unwrap().into_raw();
    let method_name_call_from_java_with_byte_array = CString::new("call_from_java_with_byte_array").ok().unwrap().into_raw();
    let method_name_call_from_java_with_byte_array_arg = CString::new("call_from_java_with_byte_array_arg").ok().unwrap().into_raw();
    
    // Native function signatures
    let method_sign_call_from_java = CString::new("()V").ok().unwrap().into_raw();
    let method_sign_call_from_java_with_int = CString::new("(I)V").ok().unwrap().into_raw();
    let method_sign_call_from_java_with_byte_array = CString::new("([B)[B").ok().unwrap().into_raw();
    let method_sign_call_from_java_with_byte_array_arg = CString::new("([B)V").ok().unwrap().into_raw();

    // Rust function pointers to be called back from java
    let call_from_java_ptr = call_from_java as extern "C" fn() as *mut c_void;
    let call_from_java_with_int_ptr = call_from_java_with_int as extern "C" fn(e: &'r JNIEnv, c: jclass, i: jint) as *mut c_void;
    let call_from_java_with_byte_array_ptr = call_from_java_with_byte_array as (extern "C" fn(e: jni_JNIEnv, c: jclass, a: jni_jbyteArray) -> jni_jbyteArray) as *mut c_void;
    let call_from_java_with_byte_array_arg_ptr = call_from_java_with_byte_array_arg as extern "C" fn(e: JNIEnv, c: jclass, a: jni_jbyteArray) as *mut c_void;

    // JNINativeMethods
    let jni_native_method_call_from_java = JNINativeMethod {
            name: method_name_call_from_java,
            signature: method_sign_call_from_java,
            fnPtr: call_from_java_ptr
    };
    let jni_native_method_from_java_with_int = JNINativeMethod {
            name: method_name_call_from_java_with_int,
            signature: method_sign_call_from_java_with_int,
            fnPtr: call_from_java_with_int_ptr
    };
    let jni_native_method_from_java_with_byte_array = JNINativeMethod {
            name: method_name_call_from_java_with_byte_array,
            signature: method_sign_call_from_java_with_byte_array,
            fnPtr: call_from_java_with_byte_array_ptr
    };
    let jni_native_method_from_java_with_byte_array_arg = JNINativeMethod {
            name: method_name_call_from_java_with_byte_array_arg,
            signature: method_sign_call_from_java_with_byte_array_arg,
            fnPtr: call_from_java_with_byte_array_arg_ptr
    };

    unsafe {
        // Method to register to java
        let reg_method = (*env.as_ref().unwrap().to_owned()).RegisterNatives.unwrap();

        //Register methods
        let reg_status1 = reg_method(env, class.jvm_ptr() as *mut _jobject, &jni_native_method_call_from_java, 1);
        let reg_status2 = reg_method(env, class.jvm_ptr() as *mut _jobject, &jni_native_method_from_java_with_int, 1);
        let reg_status3 = reg_method(env, class.jvm_ptr() as *mut _jobject, &jni_native_method_from_java_with_byte_array, 1);
        let reg_status4 = reg_method(env, class.jvm_ptr() as *mut _jobject, &jni_native_method_from_java_with_byte_array_arg, 1);
        
        println!("Rust: Registered call_from_java: {}", reg_status1 == 0);
        println!("Rust: Registered call_from_java_with_int: {}", reg_status2 == 0);
        println!("Rust: Registered call_from_java_with_byte_array: {}", reg_status3 == 0);
        println!("Rust: Registered call_from_java_with_byte_array_arg: {}", reg_status4 == 0);
    }
}

extern "C" fn call_from_java() {
    println!("Rust: Call from java without any params");
}

extern "C" fn call_from_java_with_int(env: &JNIEnv, _class: jclass, n: jint) {
    println!("Rust: Call from java with int {}",n);
}

extern "C" fn call_from_java_with_byte_array(env: jni_JNIEnv, _class: jclass, a: jni_jbyteArray) -> jni_jbyteArray {
    // println!("Rust: Call from java with byte array and returns byte array too {:?}", jbyteArray);
    // First, we have to get the byte[] out of java.
    let _input = env.convert_byte_array(a).unwrap();

    // Then we have to create a new java byte[] to return.
    let buf = [5; 2000];
    let output = env.byte_array_from_slice(&buf).unwrap();
    // Finally, extract the raw pointer to return.
    output
}

extern "C" fn call_from_java_with_byte_array_arg(env: JNIEnv, _class: jclass, a: jni_jbyteArray) {
    println!("Rust: Call from java with byte array and returns byte array with only arg {:?}",a);
    // First, we have to get the byte[] out of java.
}

fn call_java_constructor(jvm_attachment: &JvmAttachment, class: &JvmClass) {
    println!("Rust: Calling java constructor");
    // Get the Java method.
    let jvm_method = JvmMethod::get_constructor(
        &jvm_attachment,
        &class,
        "()V",
    )
    .expect("Rust: Could not find JVM method");
    JvmMethod::call_constructor(&jvm_attachment, &class, &jvm_method, null());
}