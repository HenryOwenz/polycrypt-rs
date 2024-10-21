package main

/*
#cgo LDFLAGS: -L${SRCDIR}/../target/release -lpolycrypt_rs
#include <stdlib.h>
#include <stdint.h>

typedef struct {
    uint8_t* data;
    uintptr_t len;
} ByteArray;

ByteArray encrypt(const uint8_t* plaintext, uintptr_t plaintext_len, const uint8_t* key);
ByteArray decrypt(const uint8_t* ciphertext, uintptr_t ciphertext_len, const uint8_t* key);
void free_byte_array(ByteArray arr);
void init_logger();
char* decrypt_fields(const char* record, const char* fields_to_decrypt, const uint8_t* key);
char* encrypt_fields(const char* record, const char* fields_to_encrypt, const uint8_t* key);
void free_c_char(char* s);
*/
import "C"
import (
	"encoding/json"
	"fmt"
	"os"
	"unsafe"
)

func main() {
	// Set the RUST_LOG environment variable
	os.Setenv("RUST_LOG", "info")

	// Initialize the logger
	C.init_logger()

	plaintext := []byte("Hello, world!")
	key := make([]byte, 32)
	// In a real scenario, use a proper key generation method

	cPlaintext := (*C.uint8_t)(unsafe.Pointer(&plaintext[0]))
	cKey := (*C.uint8_t)(unsafe.Pointer(&key[0]))

	encryptedResult := C.encrypt(cPlaintext, C.uintptr_t(len(plaintext)), cKey)
	defer C.free_byte_array(encryptedResult)

	encrypted := C.GoBytes(unsafe.Pointer(encryptedResult.data), C.int(encryptedResult.len))

	fmt.Printf("Plaintext: %s\n", plaintext)
	fmt.Printf("Encrypted: %v\n", encrypted)
	fmt.Printf("Encrypted length: %d\n", len(encrypted))

	cEncrypted := (*C.uint8_t)(unsafe.Pointer(&encrypted[0]))
	decryptedResult := C.decrypt(cEncrypted, C.uintptr_t(len(encrypted)), cKey)
	defer C.free_byte_array(decryptedResult)

	decrypted := C.GoBytes(unsafe.Pointer(decryptedResult.data), C.int(decryptedResult.len))

	fmt.Printf("Decrypted: %s\n", decrypted)
	fmt.Printf("Decrypted length: %d\n", len(decrypted))

	// Test encrypt_fields and decrypt_fields
	record := map[string]interface{}{
		"id":             "1234",
		"name":           "John Doe",
		"sensitive_data": "This is sensitive information",
		"array_field":    []string{"item1", "item2", "item3"},
	}
	fieldsToEncrypt := []string{"sensitive_data", "array_field"}

	fmt.Printf("Original record: %v\n", record)

	encryptedRecord := encryptFields(record, fieldsToEncrypt, key)
	fmt.Printf("Encrypted record: %v\n", encryptedRecord)

	decryptedRecord := decryptFields(encryptedRecord, fieldsToEncrypt, key)
	fmt.Printf("Decrypted record: %v\n", decryptedRecord)
}

func decryptFields(record map[string]interface{}, fieldsToDecrypt []string, key []byte) map[string]interface{} {
	recordJSON, _ := json.Marshal(record)
	fieldsJSON, _ := json.Marshal(fieldsToDecrypt)
	cRecord := C.CString(string(recordJSON))
	cFields := C.CString(string(fieldsJSON))
	cKey := (*C.uint8_t)(unsafe.Pointer(&key[0]))
	defer C.free(unsafe.Pointer(cRecord))
	defer C.free(unsafe.Pointer(cFields))

	cResult := C.decrypt_fields(cRecord, cFields, cKey)
	defer C.free_c_char(cResult)

	var result map[string]interface{}
	json.Unmarshal([]byte(C.GoString(cResult)), &result)
	return result
}

func encryptFields(record map[string]interface{}, fieldsToEncrypt []string, key []byte) map[string]interface{} {
	recordJSON, _ := json.Marshal(record)
	fieldsJSON, _ := json.Marshal(fieldsToEncrypt)
	cRecord := C.CString(string(recordJSON))
	cFields := C.CString(string(fieldsJSON))
	cKey := (*C.uint8_t)(unsafe.Pointer(&key[0]))
	defer C.free(unsafe.Pointer(cRecord))
	defer C.free(unsafe.Pointer(cFields))

	cResult := C.encrypt_fields(cRecord, cFields, cKey)
	defer C.free_c_char(cResult)

	var result map[string]interface{}
	json.Unmarshal([]byte(C.GoString(cResult)), &result)
	return result
}
