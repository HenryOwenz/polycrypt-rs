package polycrypt

/*
#cgo LDFLAGS: -L${SRCDIR} -lpolycrypt_rs
#include <stdlib.h>
#include <stdint.h>

typedef struct {
    uint8_t* data;
    uintptr_t len;
} ByteArray;

typedef struct {
    ByteArray data;
    int32_t error_code;
} FFIResult;

FFIResult encrypt(const uint8_t* plaintext, uintptr_t plaintext_len, const uint8_t* key);
FFIResult decrypt(const uint8_t* ciphertext, uintptr_t ciphertext_len, const uint8_t* key);
FFIResult encrypt_fields(const char* record, const char* fields_to_encrypt, const uint8_t* key);
FFIResult decrypt_fields(const uint8_t* encrypted, uintptr_t encrypted_len, const char* fields_to_decrypt, const uint8_t* key);
FFIResult encrypt_fields_in_batch(const char* records, const char* fields_to_encrypt, const uint8_t* key);
FFIResult decrypt_fields_in_batch(const uint8_t* encrypted, uintptr_t encrypted_len, const char* fields_to_decrypt, const uint8_t* key);
void free_ffi_result(FFIResult result);
void init_logger();
*/
import "C"
import (
	"encoding/json"
	"errors"
	"unsafe"
)

type PolyCrypt struct {
	key []byte
}

func NewPolyCrypt(key []byte) *PolyCrypt {
	return &PolyCrypt{key: key}
}

func InitLogger() {
	C.init_logger()
}

func (pc *PolyCrypt) Encrypt(plaintext []byte) ([]byte, error) {
	result := C.encrypt((*C.uint8_t)(&plaintext[0]), C.uintptr_t(len(plaintext)), (*C.uint8_t)(&pc.key[0]))
	defer C.free_ffi_result(result)

	if result.error_code != 0 {
		return nil, errors.New("encryption failed")
	}

	return C.GoBytes(unsafe.Pointer(result.data.data), C.int(result.data.len)), nil
}

func (pc *PolyCrypt) Decrypt(ciphertext []byte) ([]byte, error) {
	result := C.decrypt((*C.uint8_t)(&ciphertext[0]), C.uintptr_t(len(ciphertext)), (*C.uint8_t)(&pc.key[0]))
	defer C.free_ffi_result(result)

	if result.error_code != 0 {
		return nil, errors.New("decryption failed")
	}

	return C.GoBytes(unsafe.Pointer(result.data.data), C.int(result.data.len)), nil
}

func (pc *PolyCrypt) EncryptFields(record map[string]interface{}, fieldsToEncrypt []string) (map[string]interface{}, error) {
	recordJSON, err := json.Marshal(record)
	if err != nil {
		return nil, err
	}

	fieldsJSON, err := json.Marshal(fieldsToEncrypt)
	if err != nil {
		return nil, err
	}

	cRecord := C.CString(string(recordJSON))
	defer C.free(unsafe.Pointer(cRecord))

	cFields := C.CString(string(fieldsJSON))
	defer C.free(unsafe.Pointer(cFields))

	result := C.encrypt_fields(cRecord, cFields, (*C.uint8_t)(&pc.key[0]))
	defer C.free_ffi_result(result)

	if result.error_code != 0 {
		return nil, errors.New("field encryption failed")
	}

	encryptedJSON := C.GoBytes(unsafe.Pointer(result.data.data), C.int(result.data.len))
	var encryptedRecord map[string]interface{}
	err = json.Unmarshal(encryptedJSON, &encryptedRecord)
	if err != nil {
		return nil, err
	}

	return encryptedRecord, nil
}

func (pc *PolyCrypt) DecryptFields(encryptedRecord map[string]interface{}, fieldsToDecrypt []string) (map[string]interface{}, error) {
	encryptedJSON, err := json.Marshal(encryptedRecord)
	if err != nil {
		return nil, err
	}

	fieldsJSON, err := json.Marshal(fieldsToDecrypt)
	if err != nil {
		return nil, err
	}

	cFields := C.CString(string(fieldsJSON))
	defer C.free(unsafe.Pointer(cFields))

	result := C.decrypt_fields((*C.uint8_t)(&encryptedJSON[0]), C.uintptr_t(len(encryptedJSON)), cFields, (*C.uint8_t)(&pc.key[0]))
	defer C.free_ffi_result(result)

	if result.error_code != 0 {
		return nil, errors.New("field decryption failed")
	}

	decryptedJSON := C.GoBytes(unsafe.Pointer(result.data.data), C.int(result.data.len))
	var decryptedRecord map[string]interface{}
	err = json.Unmarshal(decryptedJSON, &decryptedRecord)
	if err != nil {
		return nil, err
	}

	// Convert []interface{} back to []string for array fields
	for _, field := range fieldsToDecrypt {
		if arr, ok := decryptedRecord[field].([]interface{}); ok {
			strArr := make([]string, len(arr))
			for i, v := range arr {
				strArr[i] = v.(string)
			}
			decryptedRecord[field] = strArr
		}
	}

	return decryptedRecord, nil
}

func (pc *PolyCrypt) EncryptFieldsInBatch(records []map[string]interface{}, fieldsToEncrypt []string) ([]map[string]interface{}, error) {
	recordsJSON, err := json.Marshal(records)
	if err != nil {
		return nil, err
	}

	fieldsJSON, err := json.Marshal(fieldsToEncrypt)
	if err != nil {
		return nil, err
	}

	cRecords := C.CString(string(recordsJSON))
	defer C.free(unsafe.Pointer(cRecords))

	cFields := C.CString(string(fieldsJSON))
	defer C.free(unsafe.Pointer(cFields))

	result := C.encrypt_fields_in_batch(cRecords, cFields, (*C.uint8_t)(&pc.key[0]))
	defer C.free_ffi_result(result)

	if result.error_code != 0 {
		return nil, errors.New("batch field encryption failed")
	}

	encryptedJSON := C.GoBytes(unsafe.Pointer(result.data.data), C.int(result.data.len))
	var encryptedRecords []map[string]interface{}
	err = json.Unmarshal(encryptedJSON, &encryptedRecords)
	if err != nil {
		return nil, err
	}

	return encryptedRecords, nil
}

func (pc *PolyCrypt) DecryptFieldsInBatch(encryptedRecords []map[string]interface{}, fieldsToDecrypt []string) ([]map[string]interface{}, error) {
	encryptedJSON, err := json.Marshal(encryptedRecords)
	if err != nil {
		return nil, err
	}

	fieldsJSON, err := json.Marshal(fieldsToDecrypt)
	if err != nil {
		return nil, err
	}

	cFields := C.CString(string(fieldsJSON))
	defer C.free(unsafe.Pointer(cFields))

	result := C.decrypt_fields_in_batch((*C.uint8_t)(&encryptedJSON[0]), C.uintptr_t(len(encryptedJSON)), cFields, (*C.uint8_t)(&pc.key[0]))
	defer C.free_ffi_result(result)

	if result.error_code != 0 {
		return nil, errors.New("batch field decryption failed")
	}

	decryptedJSON := C.GoBytes(unsafe.Pointer(result.data.data), C.int(result.data.len))
	var decryptedRecords []map[string]interface{}
	err = json.Unmarshal(decryptedJSON, &decryptedRecords)
	if err != nil {
		return nil, err
	}

	// Convert []interface{} back to []string for array fields in each record
	for _, record := range decryptedRecords {
		for _, field := range fieldsToDecrypt {
			if arr, ok := record[field].([]interface{}); ok {
				strArr := make([]string, len(arr))
				for i, v := range arr {
					strArr[i] = v.(string)
				}
				record[field] = strArr
			}
		}
	}

	return decryptedRecords, nil
}
