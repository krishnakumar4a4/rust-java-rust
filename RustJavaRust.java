/**
 * Compile this file with:
 *
 *     javac Test.java
 *
 * The Java method signatures can be seen using the Java command-line tool `javap`:
 *
 *     javap -s Test.class
 */
class RustJavaRust {
    private int uid = 10;
    public native void call_from_java();
    public native void call_from_java_with_int(int a);
    public native byte[] call_from_java_with_byte_array(byte[] input);
    public native void call_from_java_with_byte_array_arg(byte[] input);

    public RustJavaRust() {
        System.out.println("Java: Constructor called");
        call_rust();
    }
    public void call_rust() {  
        System.out.println("Java: Calling rust methods");
        call_from_java();
        call_from_java_with_int(uid);
        call_from_java_with_byte_array_arg("test".getBytes());
        byte[] outputByte = call_from_java_with_byte_array("byte".getBytes());
        System.out.println("Java: Returned array length from rust is "+outputByte.length);
    }

    public static void main(String [] args) {
    }

    public static void println(Object object) {
        System.out.println(object);
    }

    public static boolean static_method_that_returns_a_boolean(boolean arg) {
        return !arg;
    }

    public static byte static_method_that_returns_a_byte() {
        return 42;
    }

    public static char static_method_that_returns_a_char() {
        return 'a';
    }

    public static double static_method_that_returns_a_double() {
        return 42.0f;
    }

    public static float static_method_that_returns_a_float() {
        return 42.0f;
    }

    public static int static_method_that_returns_a_int() {
        return 42;
    }

    public static long static_method_that_returns_a_long() {
        return 42L;
    }

    public static String static_method_that_returns_a_string() {
        return new String("Foo");
    }

    public static String static_method_that_returns_an_interned_string() {
        return "Foo";
    }

    public static void static_void_method() {
        System.out.println(":)");
    }
}
