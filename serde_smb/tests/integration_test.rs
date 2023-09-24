use smb3::*;

#[test]
fn negotiate_request() {
    let header = RequestHeader {
        protocol_id: ProtocolId::new(),
        header_length: 64,
        credit_charge: Credits(0),
        channel_sequence: 0,
        command: Command::Negotiate,
        credits_requested: Credits(10),
        flags: HeaderFlags::new(),
        chain_offset: 0,
        message_id: MessageId(0),
        process_id: ProcessId(0xb75),
        tree_id: TreeId(0),
        session_id: SessionId(0),
        signature: Signature([0; 16]),
    };
    let req = NegotiateRequest {
        size: 0x24,
        dialect_count: 5,
        security_mode: SecurityMode::SIGNING_ENABLED,
        reserved: 0,
        capabilities: Capabilities::DFS
            | Capabilities::LEASING
            | Capabilities::LARGE_MTU
            | Capabilities::MULTI_CHANNEL
            | Capabilities::PERSISTENT_HANDLES
            | Capabilities::DIRECTORY_LEASING
            | Capabilities::ENCRYPTION,
        client_guid: Uuid {
            data1: u32::from_le_bytes([0xb2, 0x4b, 0xdf, 0xa8]),
            data2: u16::from_le_bytes([0x77, 0x93]),
            data3: u16::from_le_bytes([0xe6, 0x11]),
            data4: [0xa0, 0x1d, 0x00, 0x0c, 0x29, 0x61, 0xf5, 0x5f],
        },
        negotiate_context_offset: 0x70,
        negotiate_context_count: 2,
        dialects: vec![
            Dialect::Smb2_0_2,
            Dialect::Smb2_1,
            Dialect::Smb3_0,
            Dialect::Smb3_0_2,
            Dialect::Smb3_1_1,
        ],
        negotiate_contexts: vec![
            NegotiateContext::Smb2PreauthIntegrityCapabilities(Smb2PreauthIntegrityCapabilities {
                data_length: 38,
                reserved: 0,
                hash_algorithm_count: 1,
                salt_length: 32,
                hash_algorithms: vec![HashAlgorithm::Sha512],
                salt: vec![
                    0xd3, 0xe0, 0xee, 0xb4, 0xd9, 0xee, 0xe0, 0x3b, 0xc8, 0x5d, 0x56, 0xc6, 0x1b,
                    0x56, 0x7f, 0xf6, 0x6c, 0x56, 0x7e, 0x86, 0x82, 0xd1, 0x38, 0xcb, 0x24, 0x8b,
                    0x87, 0x73, 0x8f, 0xab, 0x80, 0xb4,
                ],
            }),
            NegotiateContext::Smb2EncryptionCapabilities(Smb2EncryptionCapabilities {
                data_length: 6,
                reserved: 0,
                cipher_count: 2,
                ciphers: vec![CipherId::Aes128Gcm, CipherId::Aes128Ccm],
            }),
        ],
    };

    let actual = serde_smb::to_vec(&(&header, &req)).unwrap();

    let expected = [
        0xfe, 0x53, 0x4d, 0x42, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x75, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x24, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00, 0x00, 0x7f, 0x00, 0x00,
        0x00, 0xb2, 0x4b, 0xdf, 0xa8, 0x77, 0x93, 0xe6, 0x11, 0xa0, 0x1d, 0x00, 0x0c, 0x29, 0x61,
        0xf5, 0x5f, 0x70, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x02, 0x02, 0x10, 0x02, 0x00,
        0x03, 0x02, 0x03, 0x11, 0x03, 0x00, 0x00, 0x01, 0x00, 0x26, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x20, 0x00, 0x01, 0x00, 0xd3, 0xe0, 0xee, 0xb4, 0xd9, 0xee, 0xe0, 0x3b, 0xc8,
        0x5d, 0x56, 0xc6, 0x1b, 0x56, 0x7f, 0xf6, 0x6c, 0x56, 0x7e, 0x86, 0x82, 0xd1, 0x38, 0xcb,
        0x24, 0x8b, 0x87, 0x73, 0x8f, 0xab, 0x80, 0xb4, 0x00, 0x00, 0x02, 0x00, 0x06, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x02, 0x00, 0x02, 0x00, 0x01, 0x00,
    ];
    assert!(
        &expected[..] == &actual[..],
        "\nexpected = {expected:x?}\nactual   = {actual:x?}"
    );

    let deserialized: (RequestHeader, NegotiateRequest) =
        serde_smb::from_slice(&expected[..]).unwrap();
    assert_eq!(deserialized, (header, req));
}

#[test]
fn negotiate_response() {
    let header = ResponseHeader {
        protocol_id: ProtocolId::new(),
        header_length: 64,
        credit_charge: Credits(0),
        nt_status: NtStatus::Success,
        command: Command::Negotiate,
        credits_granted: Credits(1),
        flags: HeaderFlags::new().with_response(true),
        chain_offset: 0,
        message_id: MessageId(0),
        process_id: ProcessId(0),
        tree_id: TreeId(0),
        session_id: SessionId(0),
        signature: Signature([0; 16]),
    };
    let req = NegotiateResponse {
        size: 0x41,
        security_mode: SecurityMode::SIGNING_ENABLED,
        dialect: Dialect::Smb3_1_1,
        negotiate_context_count: 1,
        server_guid: Uuid {
            data1: u32::from_le_bytes([0x6e, 0x61, 0x73, 0x75]),
            data2: 0,
            data3: 0,
            data4: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        },
        capabilities: Capabilities::DFS | Capabilities::LEASING | Capabilities::LARGE_MTU,
        max_transaction_size: 8388608,
        max_read_size: 8388608,
        max_write_size: 8388608,
        current_time: Time([0xd8, 0x68, 0x92, 0x79, 0x17, 0xed, 0xd9, 0x01]),
        boot_time: Time([0; 8]),
        blob_offset: 0x80,
        blob_length: 74,
        negotiate_context_offset: 0xd0,
        security_blob: vec![
            0x60, 0x48, 0x06, 0x06, 0x2b, 0x06, 0x01, 0x05, 0x05, 0x02, 0xa0, 0x3e, 0x30, 0x3c,
            0xa0, 0x0e, 0x30, 0x0c, 0x06, 0x0a, 0x2b, 0x06, 0x01, 0x04, 0x01, 0x82, 0x37, 0x02,
            0x02, 0x0a, 0xa3, 0x2a, 0x30, 0x28, 0xa0, 0x26, 0x1b, 0x24, 0x6e, 0x6f, 0x74, 0x5f,
            0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x5f, 0x69, 0x6e, 0x5f, 0x52, 0x46, 0x43,
            0x34, 0x31, 0x37, 0x38, 0x40, 0x70, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x5f, 0x69, 0x67,
            0x6e, 0x6f, 0x72, 0x65,
        ],
        negotiate_contexts: vec![NegotiateContext::Smb2PreauthIntegrityCapabilities(
            Smb2PreauthIntegrityCapabilities {
                data_length: 38,
                reserved: 0,
                hash_algorithm_count: 1,
                salt_length: 32,
                hash_algorithms: vec![HashAlgorithm::Sha512],
                salt: vec![
                    0x80, 0x13, 0x36, 0x10, 0xa3, 0xb5, 0xab, 0xb5, 0xe4, 0x02, 0xd7, 0xc8, 0x3f,
                    0x4e, 0xb6, 0x02, 0x06, 0x6c, 0x11, 0xd6, 0xe7, 0x4d, 0x72, 0xb3, 0x25, 0xe9,
                    0x29, 0x2e, 0xee, 0x82, 0x40, 0xab,
                ],
            },
        )],
    };

    let actual = serde_smb::to_vec(&(&header, &req)).unwrap();

    let expected = [
        0xfe, 0x53, 0x4d, 0x42, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x01, 0x00, 0x11, 0x03, 0x01, 0x00, 0x6e, 0x61, 0x73,
        0x75, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x00, 0xd8,
        0x68, 0x92, 0x79, 0x17, 0xed, 0xd9, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80, 0x00, 0x4a, 0x00, 0xd0, 0x00, 0x00, 0x00, 0x60, 0x48, 0x06, 0x06, 0x2b, 0x06, 0x01,
        0x05, 0x05, 0x02, 0xa0, 0x3e, 0x30, 0x3c, 0xa0, 0x0e, 0x30, 0x0c, 0x06, 0x0a, 0x2b, 0x06,
        0x01, 0x04, 0x01, 0x82, 0x37, 0x02, 0x02, 0x0a, 0xa3, 0x2a, 0x30, 0x28, 0xa0, 0x26, 0x1b,
        0x24, 0x6e, 0x6f, 0x74, 0x5f, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x5f, 0x69, 0x6e,
        0x5f, 0x52, 0x46, 0x43, 0x34, 0x31, 0x37, 0x38, 0x40, 0x70, 0x6c, 0x65, 0x61, 0x73, 0x65,
        0x5f, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
        0x26, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x20, 0x00, 0x01, 0x00, 0x80, 0x13, 0x36,
        0x10, 0xa3, 0xb5, 0xab, 0xb5, 0xe4, 0x02, 0xd7, 0xc8, 0x3f, 0x4e, 0xb6, 0x02, 0x06, 0x6c,
        0x11, 0xd6, 0xe7, 0x4d, 0x72, 0xb3, 0x25, 0xe9, 0x29, 0x2e, 0xee, 0x82, 0x40, 0xab,
    ];
    assert!(
        &expected[..] == &actual[..],
        "\nexpected = {expected:x?}\nactual   = {actual:x?}"
    );

    let deserialized: (ResponseHeader, NegotiateResponse) =
        serde_smb::from_slice(&expected[..]).unwrap();
    assert_eq!(deserialized, (header, req));
}

#[test]
fn session_setup_request() {
    let header = RequestHeader {
        protocol_id: ProtocolId::new(),
        header_length: 64,
        credit_charge: Credits(0),
        channel_sequence: 0,
        command: Command::SessionSetup,
        credits_requested: Credits(130),
        flags: HeaderFlags::new(),
        chain_offset: 0,
        message_id: MessageId(1),
        process_id: ProcessId(0xb75),
        tree_id: TreeId(0),
        session_id: SessionId(0),
        signature: Signature([0; 16]),
    };
    let req = SessionSetupRequest {
        size: 0x19,
        session_binding_request: false,
        security_mode: SecurityMode::SIGNING_ENABLED,
        capabilities: Capabilities::DFS,
        channel: 0,
        blob_offset: 0x58,
        blob_length: 32,
        previous_session_id: SessionId(0),
        security_blob: vec![
            0x4e, 0x54, 0x4c, 0x4d, 0x53, 0x53, 0x50, 0x00, 0x01, 0x00, 0x00, 0x00, 0x25, 0x02,
            0x08, 0xe0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ],
    };

    let actual = serde_smb::to_vec(&(&header, &req)).unwrap();

    let expected = [
        0xfe, 0x53, 0x4d, 0x42, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x82,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x75, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x19, 0x00, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x58, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4e, 0x54,
        0x4c, 0x4d, 0x53, 0x53, 0x50, 0x00, 0x01, 0x00, 0x00, 0x00, 0x25, 0x02, 0x08, 0xe0, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    assert!(
        &expected[..] == &actual[..],
        "\nexpected = {expected:x?}\nactual   = {actual:x?}"
    );

    let deserialized: (RequestHeader, SessionSetupRequest) =
        serde_smb::from_slice(&expected[..]).unwrap();

    assert_eq!(deserialized, (header, req));
}
