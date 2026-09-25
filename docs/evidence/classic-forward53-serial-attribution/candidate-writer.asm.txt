Disassembly of section .text:

0000000000151a50 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2>:
  151a50:	55                                              	push   %rbp
  151a51:	41 57                                           	push   %r15
  151a53:	41 56                                           	push   %r14
  151a55:	41 55                                           	push   %r13
  151a57:	41 54                                           	push   %r12
  151a59:	53                                              	push   %rbx
  151a5a:	48 81 ec 78 04 00 00                            	sub    $0x478,%rsp
  151a61:	49 89 fc                                        	mov    %rdi,%r12
  151a64:	89 74 24 14                                     	mov    %esi,0x14(%rsp)
  151a68:	89 54 24 3c                                     	mov    %edx,0x3c(%rsp)
  151a6c:	49 81 f9 ff ff 00 00                            	cmp    $0xffff,%r9
  151a73:	76 13                                           	jbe    151a88 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x38>
  151a75:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
  151a7f:	49 89 04 24                                     	mov    %rax,(%r12)
  151a83:	e9 bc 01 00 00                                  	jmp    151c44 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1f4>
  151a88:	4c 89 cb                                        	mov    %r9,%rbx
  151a8b:	41 89 cf                                        	mov    %ecx,%r15d
  151a8e:	4d 89 c6                                        	mov    %r8,%r14
  151a91:	4c 8b 8c 24 b8 04 00 00                         	mov    0x4b8(%rsp),%r9
  151a99:	4c 8b 84 24 b0 04 00 00                         	mov    0x4b0(%rsp),%r8
  151aa1:	48 8d bc 24 10 01 00 00                         	lea    0x110(%rsp),%rdi
  151aa9:	89 d9                                           	mov    %ebx,%ecx
  151aab:	e8 f0 94 f4 ff                                  	call   9afa0 <emuella_j2k_codestream::scalable_lossless::execution_requirements::<false>>
  151ab0:	48 8b 84 24 10 01 00 00                         	mov    0x110(%rsp),%rax
  151ab8:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
  151abc:	74 36                                           	je     151af4 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xa4>
  151abe:	48 8b 8c 24 38 01 00 00                         	mov    0x138(%rsp),%rcx
  151ac6:	f3 0f 6f 84 24 18 01 00 00                      	movdqu 0x118(%rsp),%xmm0
  151acf:	f3 0f 6f 8c 24 28 01 00 00                      	movdqu 0x128(%rsp),%xmm1
  151ad8:	49 89 04 24                                     	mov    %rax,(%r12)
  151adc:	f3 41 0f 7f 44 24 08                            	movdqu %xmm0,0x8(%r12)
  151ae3:	f3 41 0f 7f 4c 24 18                            	movdqu %xmm1,0x18(%r12)
  151aea:	49 89 4c 24 28                                  	mov    %rcx,0x28(%r12)
  151aef:	e9 50 01 00 00                                  	jmp    151c44 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1f4>
  151af4:	48 8b 94 24 38 01 00 00                         	mov    0x138(%rsp),%rdx
  151afc:	41 0f b6 f7                                     	movzbl %r15b,%esi
  151b00:	40 80 fe 10                                     	cmp    $0x10,%sil
  151b04:	74 09                                           	je     151b0f <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xbf>
  151b06:	83 fe 08                                        	cmp    $0x8,%esi
  151b09:	0f 85 f8 00 00 00                               	jne    151c07 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1b7>
  151b0f:	48 83 fb 08                                     	cmp    $0x8,%rbx
  151b13:	0f 94 c0                                        	sete   %al
  151b16:	41 80 ff 10                                     	cmp    $0x10,%r15b
  151b1a:	0f 95 c1                                        	setne  %cl
  151b1d:	84 c1                                           	test   %al,%cl
  151b1f:	0f 85 e2 00 00 00                               	jne    151c07 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1b7>
  151b25:	48 89 b4 24 20 02 00 00                         	mov    %rsi,0x220(%rsp)
  151b2d:	48 89 94 24 d0 00 00 00                         	mov    %rdx,0xd0(%rsp)
  151b35:	44 89 f8                                        	mov    %r15d,%eax
  151b38:	c0 e8 03                                        	shr    $0x3,%al
  151b3b:	44 0f b6 c0                                     	movzbl %al,%r8d
  151b3f:	8b 44 24 14                                     	mov    0x14(%rsp),%eax
  151b43:	8b 54 24 3c                                     	mov    0x3c(%rsp),%edx
  151b47:	49 89 d5                                        	mov    %rdx,%r13
  151b4a:	4c 0f af e8                                     	imul   %rax,%r13
  151b4e:	48 89 9c 24 88 00 00 00                         	mov    %rbx,0x88(%rsp)
  151b56:	49 89 d9                                        	mov    %rbx,%r9
  151b59:	49 c1 e1 05                                     	shl    $0x5,%r9
  151b5d:	4d 89 f2                                        	mov    %r14,%r10
  151b60:	4b 8d 0c 0e                                     	lea    (%r14,%r9,1),%rcx
  151b64:	48 89 4c 24 60                                  	mov    %rcx,0x60(%rsp)
  151b69:	48 89 44 24 08                                  	mov    %rax,0x8(%rsp)
  151b6e:	48 8d 48 ff                                     	lea    -0x1(%rax),%rcx
  151b72:	48 89 54 24 28                                  	mov    %rdx,0x28(%rsp)
  151b77:	48 8d 72 ff                                     	lea    -0x1(%rdx),%rsi
  151b7b:	0f 1f 44 00 00                                  	nopl   0x0(%rax,%rax,1)
  151b80:	4d 85 c9                                        	test   %r9,%r9
  151b83:	0f 84 d0 00 00 00                               	je     151c59 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x209>
  151b89:	4d 8b 5a 18                                     	mov    0x18(%r10),%r11
  151b8d:	48 89 c8                                        	mov    %rcx,%rax
  151b90:	49 f7 e3                                        	mul    %r11
  151b93:	0f 80 dc fe ff ff                               	jo     151a75 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x25>
  151b99:	48 89 c7                                        	mov    %rax,%rdi
  151b9c:	4c 01 c7                                        	add    %r8,%rdi
  151b9f:	0f 82 d0 fe ff ff                               	jb     151a75 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x25>
  151ba5:	49 8b 5a 10                                     	mov    0x10(%r10),%rbx
  151ba9:	48 89 f0                                        	mov    %rsi,%rax
  151bac:	48 f7 e3                                        	mul    %rbx
  151baf:	0f 80 c0 fe ff ff                               	jo     151a75 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x25>
  151bb5:	48 01 f8                                        	add    %rdi,%rax
  151bb8:	0f 82 b7 fe ff ff                               	jb     151a75 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x25>
  151bbe:	4d 39 c3                                        	cmp    %r8,%r11
  151bc1:	72 13                                           	jb     151bd6 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x186>
  151bc3:	48 39 fb                                        	cmp    %rdi,%rbx
  151bc6:	72 0e                                           	jb     151bd6 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x186>
  151bc8:	49 83 c1 e0                                     	add    $0xffffffffffffffe0,%r9
  151bcc:	49 39 42 08                                     	cmp    %rax,0x8(%r10)
  151bd0:	4d 8d 52 20                                     	lea    0x20(%r10),%r10
  151bd4:	73 aa                                           	jae    151b80 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x130>
  151bd6:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
  151be0:	48 ff c8                                        	dec    %rax
  151be3:	49 89 04 24                                     	mov    %rax,(%r12)
  151be7:	49 c7 44 24 08 00 00 00 00                      	movq   $0x0,0x8(%r12)
  151bf0:	48 8d 05 8d 1f ec ff                            	lea    -0x13e073(%rip),%rax        # 13b84 <crossbeam_epoch::guard::unprotected::UNPROTECTED+0xe24>
  151bf7:	49 89 44 24 18                                  	mov    %rax,0x18(%r12)
  151bfc:	49 c7 44 24 20 43 00 00 00                      	movq   $0x43,0x20(%r12)
  151c05:	eb 2f                                           	jmp    151c36 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1e6>
  151c07:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
  151c11:	48 ff c8                                        	dec    %rax
  151c14:	49 89 04 24                                     	mov    %rax,(%r12)
  151c18:	49 c7 44 24 08 00 00 00 00                      	movq   $0x0,0x8(%r12)
  151c21:	48 8d 05 af 1f ec ff                            	lea    -0x13e051(%rip),%rax        # 13bd7 <crossbeam_epoch::guard::unprotected::UNPROTECTED+0xe77>
  151c28:	49 89 44 24 18                                  	mov    %rax,0x18(%r12)
  151c2d:	49 c7 44 24 20 33 00 00 00                      	movq   $0x33,0x20(%r12)
  151c36:	66 41 c7 44 24 28 04 00                         	movw   $0x4,0x28(%r12)
  151c3e:	41 c6 44 24 2c 0a                               	movb   $0xa,0x2c(%r12)
  151c44:	4c 89 e0                                        	mov    %r12,%rax
  151c47:	48 81 c4 78 04 00 00                            	add    $0x478,%rsp
  151c4e:	5b                                              	pop    %rbx
  151c4f:	41 5c                                           	pop    %r12
  151c51:	41 5d                                           	pop    %r13
  151c53:	41 5e                                           	pop    %r14
  151c55:	41 5f                                           	pop    %r15
  151c57:	5d                                              	pop    %rbp
  151c58:	c3                                              	ret
  151c59:	48 8d bc 24 10 01 00 00                         	lea    0x110(%rsp),%rdi
  151c61:	ba 08 00 00 00                                  	mov    $0x8,%edx
  151c66:	b9 18 00 00 00                                  	mov    $0x18,%ecx
  151c6b:	48 8b 9c 24 88 00 00 00                         	mov    0x88(%rsp),%rbx
  151c73:	48 89 de                                        	mov    %rbx,%rsi
  151c76:	e8 f5 90 ff ff                                  	call   14ad70 <<alloc::raw_vec::RawVecInner>::try_allocate_in>
  151c7b:	48 8b bc 24 18 01 00 00                         	mov    0x118(%rsp),%rdi
  151c83:	80 bc 24 10 01 00 00 00                         	cmpb   $0x0,0x110(%rsp)
  151c8b:	0f 85 bd 03 00 00                               	jne    15204e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x5fe>
  151c91:	48 8b 84 24 20 01 00 00                         	mov    0x120(%rsp),%rax
  151c99:	48 89 7c 24 70                                  	mov    %rdi,0x70(%rsp)
  151c9e:	48 89 44 24 78                                  	mov    %rax,0x78(%rsp)
  151ca3:	48 c7 84 24 80 00 00 00 00 00 00 00             	movq   $0x0,0x80(%rsp)
  151caf:	48 85 db                                        	test   %rbx,%rbx
  151cb2:	4c 89 24 24                                     	mov    %r12,(%rsp)
  151cb6:	0f 84 10 04 00 00                               	je     1520cc <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x67c>
  151cbc:	4c 89 f2                                        	mov    %r14,%rdx
  151cbf:	41 8d 4f ff                                     	lea    -0x1(%r15),%ecx
  151cc3:	b8 ff ff ff ff                                  	mov    $0xffffffff,%eax
  151cc8:	d3 e0                                           	shl    %cl,%eax
  151cca:	89 44 24 40                                     	mov    %eax,0x40(%rsp)
  151cce:	4c 89 bc 24 98 00 00 00                         	mov    %r15,0x98(%rsp)
  151cd6:	4c 89 ac 24 90 00 00 00                         	mov    %r13,0x90(%rsp)
  151cde:	eb 4a                                           	jmp    151d2a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x2da>
  151ce0:	48 8b 54 24 30                                  	mov    0x30(%rsp),%rdx
  151ce5:	48 83 c2 20                                     	add    $0x20,%rdx
  151ce9:	48 8b 44 24 78                                  	mov    0x78(%rsp),%rax
  151cee:	48 8d 0c 5b                                     	lea    (%rbx,%rbx,2),%rcx
  151cf2:	4c 89 3c c8                                     	mov    %r15,(%rax,%rcx,8)
  151cf6:	4c 89 74 c8 08                                  	mov    %r14,0x8(%rax,%rcx,8)
  151cfb:	4c 89 6c c8 10                                  	mov    %r13,0x10(%rax,%rcx,8)
  151d00:	48 ff c3                                        	inc    %rbx
  151d03:	48 89 9c 24 80 00 00 00                         	mov    %rbx,0x80(%rsp)
  151d0b:	48 3b 54 24 60                                  	cmp    0x60(%rsp),%rdx
  151d10:	4c 8b 24 24                                     	mov    (%rsp),%r12
  151d14:	4c 8b bc 24 98 00 00 00                         	mov    0x98(%rsp),%r15
  151d1c:	4c 8b ac 24 90 00 00 00                         	mov    0x90(%rsp),%r13
  151d24:	0f 84 4a 02 00 00                               	je     151f74 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x524>
  151d2a:	48 c7 84 24 b0 03 00 00 00 00 00 00             	movq   $0x0,0x3b0(%rsp)
  151d36:	48 c7 84 24 b8 03 00 00 04 00 00 00             	movq   $0x4,0x3b8(%rsp)
  151d42:	48 c7 84 24 c0 03 00 00 00 00 00 00             	movq   $0x0,0x3c0(%rsp)
  151d4e:	b8 04 00 00 00                                  	mov    $0x4,%eax
  151d53:	4d 85 ed                                        	test   %r13,%r13
  151d56:	0f 85 83 01 00 00                               	jne    151edf <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x48f>
  151d5c:	83 7c 24 28 00                                  	cmpl   $0x0,0x28(%rsp)
  151d61:	48 89 54 24 30                                  	mov    %rdx,0x30(%rsp)
  151d66:	0f 84 d2 01 00 00                               	je     151f3e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x4ee>
  151d6c:	83 7c 24 08 00                                  	cmpl   $0x0,0x8(%rsp)
  151d71:	0f 84 c7 01 00 00                               	je     151f3e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x4ee>
  151d77:	48 8b 4a 10                                     	mov    0x10(%rdx),%rcx
  151d7b:	48 89 4c 24 20                                  	mov    %rcx,0x20(%rsp)
  151d80:	48 8b 5a 18                                     	mov    0x18(%rdx),%rbx
  151d84:	41 80 ff 08                                     	cmp    $0x8,%r15b
  151d88:	0f 85 96 00 00 00                               	jne    151e24 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x3d4>
  151d8e:	4c 8b 72 08                                     	mov    0x8(%rdx),%r14
  151d92:	31 ed                                           	xor    %ebp,%ebp
  151d94:	45 31 ed                                        	xor    %r13d,%r13d
  151d97:	31 c9                                           	xor    %ecx,%ecx
  151d99:	eb 1f                                           	jmp    151dba <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x36a>
  151d9b:	0f 1f 44 00 00                                  	nopl   0x0(%rax,%rax,1)
  151da0:	48 8b 6c 24 18                                  	mov    0x18(%rsp),%rbp
  151da5:	48 03 6c 24 20                                  	add    0x20(%rsp),%rbp
  151daa:	48 8b 4c 24 68                                  	mov    0x68(%rsp),%rcx
  151daf:	48 3b 4c 24 28                                  	cmp    0x28(%rsp),%rcx
  151db4:	0f 84 87 01 00 00                               	je     151f41 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x4f1>
  151dba:	48 ff c1                                        	inc    %rcx
  151dbd:	48 89 4c 24 68                                  	mov    %rcx,0x68(%rsp)
  151dc2:	4c 8b 64 24 08                                  	mov    0x8(%rsp),%r12
  151dc7:	48 89 6c 24 18                                  	mov    %rbp,0x18(%rsp)
  151dcc:	eb 1e                                           	jmp    151dec <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x39c>
  151dce:	66 90                                           	xchg   %ax,%ax
  151dd0:	44 03 7c 24 40                                  	add    0x40(%rsp),%r15d
  151dd5:	46 89 3c a8                                     	mov    %r15d,(%rax,%r13,4)
  151dd9:	49 ff c5                                        	inc    %r13
  151ddc:	4c 89 ac 24 c0 03 00 00                         	mov    %r13,0x3c0(%rsp)
  151de4:	48 01 dd                                        	add    %rbx,%rbp
  151de7:	49 ff cc                                        	dec    %r12
  151dea:	74 b4                                           	je     151da0 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x350>
  151dec:	4c 39 f5                                        	cmp    %r14,%rbp
  151def:	0f 83 2c 02 00 00                               	jae    152021 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x5d1>
  151df5:	48 8b 4c 24 30                                  	mov    0x30(%rsp),%rcx
  151dfa:	48 8b 09                                        	mov    (%rcx),%rcx
  151dfd:	44 0f b6 3c 29                                  	movzbl (%rcx,%rbp,1),%r15d
  151e02:	4c 3b ac 24 b0 03 00 00                         	cmp    0x3b0(%rsp),%r13
  151e0a:	75 c4                                           	jne    151dd0 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x380>
  151e0c:	48 8d bc 24 b0 03 00 00                         	lea    0x3b0(%rsp),%rdi
  151e14:	ff 15 46 34 12 00                               	call   *0x123446(%rip)        # 275260 <_DYNAMIC+0x6b0>
  151e1a:	48 8b 84 24 b8 03 00 00                         	mov    0x3b8(%rsp),%rax
  151e22:	eb ac                                           	jmp    151dd0 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x380>
  151e24:	48 8b 0a                                        	mov    (%rdx),%rcx
  151e27:	48 89 4c 24 18                                  	mov    %rcx,0x18(%rsp)
  151e2c:	4c 8b 72 08                                     	mov    0x8(%rdx),%r14
  151e30:	41 bc 01 00 00 00                               	mov    $0x1,%r12d
  151e36:	45 31 ed                                        	xor    %r13d,%r13d
  151e39:	31 c9                                           	xor    %ecx,%ecx
  151e3b:	eb 20                                           	jmp    151e5d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x40d>
  151e3d:	0f 1f 00                                        	nopl   (%rax)
  151e40:	4c 8b 64 24 68                                  	mov    0x68(%rsp),%r12
  151e45:	4c 03 64 24 20                                  	add    0x20(%rsp),%r12
  151e4a:	48 8b 8c 24 d8 00 00 00                         	mov    0xd8(%rsp),%rcx
  151e52:	48 3b 4c 24 28                                  	cmp    0x28(%rsp),%rcx
  151e57:	0f 84 e4 00 00 00                               	je     151f41 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x4f1>
  151e5d:	48 ff c1                                        	inc    %rcx
  151e60:	48 89 8c 24 d8 00 00 00                         	mov    %rcx,0xd8(%rsp)
  151e68:	4c 8b 7c 24 08                                  	mov    0x8(%rsp),%r15
  151e6d:	4c 89 64 24 68                                  	mov    %r12,0x68(%rsp)
  151e72:	eb 27                                           	jmp    151e9b <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x44b>
  151e74:	66 66 66 2e 0f 1f 84 00 00 00 00 00             	data16 data16 cs nopw 0x0(%rax,%rax,1)
  151e80:	03 6c 24 40                                     	add    0x40(%rsp),%ebp
  151e84:	42 89 2c a8                                     	mov    %ebp,(%rax,%r13,4)
  151e88:	49 ff c5                                        	inc    %r13
  151e8b:	4c 89 ac 24 c0 03 00 00                         	mov    %r13,0x3c0(%rsp)
  151e93:	49 01 dc                                        	add    %rbx,%r12
  151e96:	49 ff cf                                        	dec    %r15
  151e99:	74 a5                                           	je     151e40 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x3f0>
  151e9b:	49 8d 6c 24 ff                                  	lea    -0x1(%r12),%rbp
  151ea0:	4c 39 f5                                        	cmp    %r14,%rbp
  151ea3:	0f 83 81 01 00 00                               	jae    15202a <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x5da>
  151ea9:	4d 39 f4                                        	cmp    %r14,%r12
  151eac:	0f 83 81 01 00 00                               	jae    152033 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x5e3>
  151eb2:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
  151eb7:	42 0f b7 6c 21 ff                               	movzwl -0x1(%rcx,%r12,1),%ebp
  151ebd:	4c 3b ac 24 b0 03 00 00                         	cmp    0x3b0(%rsp),%r13
  151ec5:	75 b9                                           	jne    151e80 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x430>
  151ec7:	48 8d bc 24 b0 03 00 00                         	lea    0x3b0(%rsp),%rdi
  151ecf:	ff 15 8b 33 12 00                               	call   *0x12338b(%rip)        # 275260 <_DYNAMIC+0x6b0>
  151ed5:	48 8b 84 24 b8 03 00 00                         	mov    0x3b8(%rsp),%rax
  151edd:	eb a1                                           	jmp    151e80 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x430>
  151edf:	48 89 d3                                        	mov    %rdx,%rbx
  151ee2:	ba 04 00 00 00                                  	mov    $0x4,%edx
  151ee7:	41 b8 04 00 00 00                               	mov    $0x4,%r8d
  151eed:	41 b9 04 00 00 00                               	mov    $0x4,%r9d
  151ef3:	48 8d bc 24 10 01 00 00                         	lea    0x110(%rsp),%rdi
  151efb:	31 f6                                           	xor    %esi,%esi
  151efd:	4c 89 e9                                        	mov    %r13,%rcx
  151f00:	e8 0b 8d ff ff                                  	call   14ac10 <<alloc::raw_vec::RawVecInner>::finish_grow>
  151f05:	80 bc 24 10 01 00 00 00                         	cmpb   $0x0,0x110(%rsp)
  151f0d:	0f 85 a2 00 00 00                               	jne    151fb5 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x565>
  151f13:	48 8b 84 24 18 01 00 00                         	mov    0x118(%rsp),%rax
  151f1b:	48 89 84 24 b8 03 00 00                         	mov    %rax,0x3b8(%rsp)
  151f23:	4c 89 ac 24 b0 03 00 00                         	mov    %r13,0x3b0(%rsp)
  151f2b:	48 89 da                                        	mov    %rbx,%rdx
  151f2e:	83 7c 24 28 00                                  	cmpl   $0x0,0x28(%rsp)
  151f33:	48 89 54 24 30                                  	mov    %rdx,0x30(%rsp)
  151f38:	0f 85 2e fe ff ff                               	jne    151d6c <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x31c>
  151f3e:	45 31 ed                                        	xor    %r13d,%r13d
  151f41:	4c 8b bc 24 b0 03 00 00                         	mov    0x3b0(%rsp),%r15
  151f49:	4c 8b b4 24 b8 03 00 00                         	mov    0x3b8(%rsp),%r14
  151f51:	48 8b 9c 24 80 00 00 00                         	mov    0x80(%rsp),%rbx
  151f59:	48 3b 5c 24 70                                  	cmp    0x70(%rsp),%rbx
  151f5e:	0f 85 7c fd ff ff                               	jne    151ce0 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x290>
  151f64:	48 8d 7c 24 70                                  	lea    0x70(%rsp),%rdi
  151f69:	ff 15 f9 32 12 00                               	call   *0x1232f9(%rip)        # 275268 <_DYNAMIC+0x6b8>
  151f6f:	e9 6c fd ff ff                                  	jmp    151ce0 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x290>
  151f74:	48 83 fb 03                                     	cmp    $0x3,%rbx
  151f78:	0f 85 4e 01 00 00                               	jne    1520cc <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x67c>
  151f7e:	48 8b 74 24 78                                  	mov    0x78(%rsp),%rsi
  151f83:	48 8b 46 10                                     	mov    0x10(%rsi),%rax
  151f87:	48 3b 46 28                                     	cmp    0x28(%rsi),%rax
  151f8b:	75 28                                           	jne    151fb5 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x565>
  151f8d:	48 3b 46 40                                     	cmp    0x40(%rsi),%rax
  151f91:	75 22                                           	jne    151fb5 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x565>
  151f93:	48 85 c0                                        	test   %rax,%rax
  151f96:	0f 84 30 01 00 00                               	je     1520cc <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x67c>
  151f9c:	48 8b 4e 08                                     	mov    0x8(%rsi),%rcx
  151fa0:	48 8b 56 20                                     	mov    0x20(%rsi),%rdx
  151fa4:	48 8b 76 38                                     	mov    0x38(%rsi),%rsi
  151fa8:	48 83 f8 04                                     	cmp    $0x4,%rax
  151fac:	73 1a                                           	jae    151fc8 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x578>
  151fae:	31 ff                                           	xor    %edi,%edi
  151fb0:	e9 e6 00 00 00                                  	jmp    15209b <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x64b>
  151fb5:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
  151fbf:	49 89 04 24                                     	mov    %rax,(%r12)
  151fc3:	e9 55 03 00 00                                  	jmp    15231d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x8cd>
  151fc8:	48 89 c7                                        	mov    %rax,%rdi
  151fcb:	48 83 e7 fc                                     	and    $0xfffffffffffffffc,%rdi
  151fcf:	45 31 c0                                        	xor    %r8d,%r8d
  151fd2:	f3 42 0f 6f 04 81                               	movdqu (%rcx,%r8,4),%xmm0
  151fd8:	f3 42 0f 6f 0c 82                               	movdqu (%rdx,%r8,4),%xmm1
  151fde:	f3 42 0f 6f 14 86                               	movdqu (%rsi,%r8,4),%xmm2
  151fe4:	66 0f 6f d8                                     	movdqa %xmm0,%xmm3
  151fe8:	66 0f fe da                                     	paddd  %xmm2,%xmm3
  151fec:	66 0f fa d1                                     	psubd  %xmm1,%xmm2
  151ff0:	66 0f fa c1                                     	psubd  %xmm1,%xmm0
  151ff4:	66 0f fe c9                                     	paddd  %xmm1,%xmm1
  151ff8:	66 0f fe d9                                     	paddd  %xmm1,%xmm3
  151ffc:	66 0f 72 e3 02                                  	psrad  $0x2,%xmm3
  152001:	f3 42 0f 7f 1c 81                               	movdqu %xmm3,(%rcx,%r8,4)
  152007:	f3 42 0f 7f 14 82                               	movdqu %xmm2,(%rdx,%r8,4)
  15200d:	f3 42 0f 7f 04 86                               	movdqu %xmm0,(%rsi,%r8,4)
  152013:	49 83 c0 04                                     	add    $0x4,%r8
  152017:	4c 39 c7                                        	cmp    %r8,%rdi
  15201a:	75 b6                                           	jne    151fd2 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x582>
  15201c:	e9 a6 00 00 00                                  	jmp    1520c7 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x677>
  152021:	48 8d 15 30 bf 11 00                            	lea    0x11bf30(%rip),%rdx        # 26df58 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x2e0>
  152028:	eb 13                                           	jmp    15203d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x5ed>
  15202a:	48 8d 15 3f bf 11 00                            	lea    0x11bf3f(%rip),%rdx        # 26df70 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x2f8>
  152031:	eb 0a                                           	jmp    15203d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x5ed>
  152033:	4c 89 e5                                        	mov    %r12,%rbp
  152036:	48 8d 15 4b bf 11 00                            	lea    0x11bf4b(%rip),%rdx        # 26df88 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x310>
  15203d:	48 89 ef                                        	mov    %rbp,%rdi
  152040:	4c 89 f6                                        	mov    %r14,%rsi
  152043:	ff 15 c7 2d 12 00                               	call   *0x122dc7(%rip)        # 274e10 <_DYNAMIC+0x260>
  152049:	e9 61 0e 00 00                                  	jmp    152eaf <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x145f>
  15204e:	48 8b b4 24 20 01 00 00                         	mov    0x120(%rsp),%rsi
  152056:	ff 15 a4 2d 12 00                               	call   *0x122da4(%rip)        # 274e00 <_DYNAMIC+0x250>
  15205c:	49 89 c5                                        	mov    %rax,%r13
  15205f:	4d 85 ff                                        	test   %r15,%r15
  152062:	74 25                                           	je     152089 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x639>
  152064:	4c 89 f7                                        	mov    %r14,%rdi
  152067:	eb 1a                                           	jmp    152083 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x633>
  152069:	eb 02                                           	jmp    15206d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x61d>
  15206b:	eb 00                                           	jmp    15206d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x61d>
  15206d:	49 89 c5                                        	mov    %rax,%r13
  152070:	48 83 bc 24 b0 03 00 00 00                      	cmpq   $0x0,0x3b0(%rsp)
  152079:	74 0e                                           	je     152089 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x639>
  15207b:	48 8b bc 24 b8 03 00 00                         	mov    0x3b8(%rsp),%rdi
  152083:	ff 15 3f 2d 12 00                               	call   *0x122d3f(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152089:	48 8d 7c 24 70                                  	lea    0x70(%rsp),%rdi
  15208e:	e8 cd f4 f3 ff                                  	call   91560 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<emuella_j2k_accel::OutputRect>>>>
  152093:	4c 89 ef                                        	mov    %r13,%rdi
  152096:	e8 05 a1 11 00                                  	call   26c1a0 <_Unwind_Resume@plt>
  15209b:	44 8b 04 b9                                     	mov    (%rcx,%rdi,4),%r8d
  15209f:	44 8b 0c ba                                     	mov    (%rdx,%rdi,4),%r9d
  1520a3:	47 8d 14 48                                     	lea    (%r8,%r9,2),%r10d
  1520a7:	44 8b 1c be                                     	mov    (%rsi,%rdi,4),%r11d
  1520ab:	45 01 da                                        	add    %r11d,%r10d
  1520ae:	41 c1 fa 02                                     	sar    $0x2,%r10d
  1520b2:	44 89 14 b9                                     	mov    %r10d,(%rcx,%rdi,4)
  1520b6:	45 29 cb                                        	sub    %r9d,%r11d
  1520b9:	44 89 1c ba                                     	mov    %r11d,(%rdx,%rdi,4)
  1520bd:	45 29 c8                                        	sub    %r9d,%r8d
  1520c0:	44 89 04 be                                     	mov    %r8d,(%rsi,%rdi,4)
  1520c4:	48 ff c7                                        	inc    %rdi
  1520c7:	48 39 f8                                        	cmp    %rdi,%rax
  1520ca:	75 cf                                           	jne    15209b <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x64b>
  1520cc:	48 8d bc 24 10 01 00 00                         	lea    0x110(%rsp),%rdi
  1520d4:	48 8b 74 24 08                                  	mov    0x8(%rsp),%rsi
  1520d9:	48 8b 54 24 28                                  	mov    0x28(%rsp),%rdx
  1520de:	48 8b 8c 24 d0 00 00 00                         	mov    0xd0(%rsp),%rcx
  1520e6:	e8 15 f5 ff ff                                  	call   151600 <emuella_j2k_codestream::scalable_lossless::prepare_forward53>
  1520eb:	4c 8b bc 24 c0 01 00 00                         	mov    0x1c0(%rsp),%r15
  1520f3:	49 83 ff fe                                     	cmp    $0xfffffffffffffffe,%r15
  1520f7:	75 4f                                           	jne    152148 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x6f8>
  1520f9:	f3 0f 6f 84 24 10 01 00 00                      	movdqu 0x110(%rsp),%xmm0
  152102:	f3 0f 6f 8c 24 20 01 00 00                      	movdqu 0x120(%rsp),%xmm1
  15210b:	f3 0f 6f 94 24 30 01 00 00                      	movdqu 0x130(%rsp),%xmm2
  152114:	66 0f 7f 94 24 60 02 00 00                      	movdqa %xmm2,0x260(%rsp)
  15211d:	66 0f 7f 8c 24 50 02 00 00                      	movdqa %xmm1,0x250(%rsp)
  152126:	66 0f 7f 84 24 40 02 00 00                      	movdqa %xmm0,0x240(%rsp)
  15212f:	f3 41 0f 7f 54 24 20                            	movdqu %xmm2,0x20(%r12)
  152136:	f3 41 0f 7f 4c 24 10                            	movdqu %xmm1,0x10(%r12)
  15213d:	f3 41 0f 7f 04 24                               	movdqu %xmm0,(%r12)
  152143:	e9 d5 01 00 00                                  	jmp    15231d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x8cd>
  152148:	48 8d 9c 24 40 02 00 00                         	lea    0x240(%rsp),%rbx
  152150:	48 8d b4 24 10 01 00 00                         	lea    0x110(%rsp),%rsi
  152158:	4c 8b 2d 79 2c 12 00                            	mov    0x122c79(%rip),%r13        # 274dd8 <memcpy@GLIBC_2.14>
  15215f:	ba b0 00 00 00                                  	mov    $0xb0,%edx
  152164:	48 89 df                                        	mov    %rbx,%rdi
  152167:	41 ff d5                                        	call   *%r13
  15216a:	f3 0f 6f 84 24 c8 01 00 00                      	movdqu 0x1c8(%rsp),%xmm0
  152173:	f3 0f 7f 84 24 68 04 00 00                      	movdqu %xmm0,0x468(%rsp)
  15217c:	4c 8d b4 24 b0 03 00 00                         	lea    0x3b0(%rsp),%r14
  152184:	ba b0 00 00 00                                  	mov    $0xb0,%edx
  152189:	4c 89 f7                                        	mov    %r14,%rdi
  15218c:	48 89 de                                        	mov    %rbx,%rsi
  15218f:	41 ff d5                                        	call   *%r13
  152192:	4c 89 bc 24 60 04 00 00                         	mov    %r15,0x460(%rsp)
  15219a:	48 c7 84 24 e0 00 00 00 00 00 00 00             	movq   $0x0,0xe0(%rsp)
  1521a6:	48 c7 84 24 e8 00 00 00 04 00 00 00             	movq   $0x4,0xe8(%rsp)
  1521b2:	48 c7 84 24 f0 00 00 00 00 00 00 00             	movq   $0x0,0xf0(%rsp)
  1521be:	48 8d 44 24 70                                  	lea    0x70(%rsp),%rax
  1521c3:	48 89 84 24 58 03 00 00                         	mov    %rax,0x358(%rsp)
  1521cb:	4c 89 b4 24 60 03 00 00                         	mov    %r14,0x360(%rsp)
  1521d3:	4c 8d 7c 24 14                                  	lea    0x14(%rsp),%r15
  1521d8:	4c 89 bc 24 68 03 00 00                         	mov    %r15,0x368(%rsp)
  1521e0:	48 8d 44 24 3c                                  	lea    0x3c(%rsp),%rax
  1521e5:	48 89 84 24 70 03 00 00                         	mov    %rax,0x370(%rsp)
  1521ed:	48 8d 84 24 e0 00 00 00                         	lea    0xe0(%rsp),%rax
  1521f5:	48 89 84 24 78 03 00 00                         	mov    %rax,0x378(%rsp)
  1521fd:	48 8d bc 24 10 01 00 00                         	lea    0x110(%rsp),%rdi
  152205:	48 8d b4 24 58 03 00 00                         	lea    0x358(%rsp),%rsi
  15220d:	e8 3e df f7 ff                                  	call   d0150 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl::<false, false>::{closure#5}>
  152212:	48 83 bc 24 10 01 00 00 ff                      	cmpq   $0xffffffffffffffff,0x110(%rsp)
  15221b:	74 6a                                           	je     152287 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x837>
  15221d:	f3 0f 6f 84 24 10 01 00 00                      	movdqu 0x110(%rsp),%xmm0
  152226:	f3 0f 6f 8c 24 20 01 00 00                      	movdqu 0x120(%rsp),%xmm1
  15222f:	f3 0f 6f 94 24 30 01 00 00                      	movdqu 0x130(%rsp),%xmm2
  152238:	f3 41 0f 7f 54 24 20                            	movdqu %xmm2,0x20(%r12)
  15223f:	f3 41 0f 7f 4c 24 10                            	movdqu %xmm1,0x10(%r12)
  152246:	f3 41 0f 7f 04 24                               	movdqu %xmm0,(%r12)
  15224c:	48 83 bc 24 e0 00 00 00 00                      	cmpq   $0x0,0xe0(%rsp)
  152255:	74 0e                                           	je     152265 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x815>
  152257:	48 8b bc 24 e8 00 00 00                         	mov    0xe8(%rsp),%rdi
  15225f:	ff 15 63 2b 12 00                               	call   *0x122b63(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152265:	48 83 bc 24 60 04 00 00 00                      	cmpq   $0x0,0x460(%rsp)
  15226e:	0f 8e a9 00 00 00                               	jle    15231d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x8cd>
  152274:	48 8b bc 24 68 04 00 00                         	mov    0x468(%rsp),%rdi
  15227c:	ff 15 46 2b 12 00                               	call   *0x122b46(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152282:	e9 96 00 00 00                                  	jmp    15231d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x8cd>
  152287:	48 8b 84 24 60 04 00 00                         	mov    0x460(%rsp),%rax
  15228f:	48 ff c8                                        	dec    %rax
  152292:	48 83 f8 fd                                     	cmp    $0xfffffffffffffffd,%rax
  152296:	77 0e                                           	ja     1522a6 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x856>
  152298:	48 8b bc 24 68 04 00 00                         	mov    0x468(%rsp),%rdi
  1522a0:	ff 15 22 2b 12 00                               	call   *0x122b22(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  1522a6:	48 83 bc 24 e0 00 00 00 00                      	cmpq   $0x0,0xe0(%rsp)
  1522af:	74 0e                                           	je     1522bf <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x86f>
  1522b1:	48 8b bc 24 e8 00 00 00                         	mov    0xe8(%rsp),%rdi
  1522b9:	ff 15 09 2b 12 00                               	call   *0x122b09(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  1522bf:	8b 74 24 14                                     	mov    0x14(%rsp),%esi
  1522c3:	8b 54 24 3c                                     	mov    0x3c(%rsp),%edx
  1522c7:	48 8d bc 24 10 01 00 00                         	lea    0x110(%rsp),%rdi
  1522cf:	e8 5c 14 f8 ff                                  	call   d3730 <emuella_j2k_codestream::decomp_subband_specs>
  1522d4:	48 8b 84 24 10 01 00 00                         	mov    0x110(%rsp),%rax
  1522dc:	48 8b 8c 24 18 01 00 00                         	mov    0x118(%rsp),%rcx
  1522e4:	48 8b ac 24 20 01 00 00                         	mov    0x120(%rsp),%rbp
  1522ec:	4c 8b b4 24 28 01 00 00                         	mov    0x128(%rsp),%r14
  1522f4:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
  1522f8:	74 32                                           	je     15232c <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x8dc>
  1522fa:	f3 0f 6f 84 24 30 01 00 00                      	movdqu 0x130(%rsp),%xmm0
  152303:	f3 41 0f 7f 44 24 20                            	movdqu %xmm0,0x20(%r12)
  15230a:	49 89 04 24                                     	mov    %rax,(%r12)
  15230e:	49 89 4c 24 08                                  	mov    %rcx,0x8(%r12)
  152313:	49 89 6c 24 10                                  	mov    %rbp,0x10(%r12)
  152318:	4d 89 74 24 18                                  	mov    %r14,0x18(%r12)
  15231d:	48 8d 7c 24 70                                  	lea    0x70(%rsp),%rdi
  152322:	e8 39 f2 f3 ff                                  	call   91560 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<emuella_j2k_accel::OutputRect>>>>
  152327:	e9 18 f9 ff ff                                  	jmp    151c44 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1f4>
  15232c:	48 89 4c 24 40                                  	mov    %rcx,0x40(%rsp)
  152331:	48 8b 74 24 78                                  	mov    0x78(%rsp),%rsi
  152336:	48 8b 84 24 80 00 00 00                         	mov    0x80(%rsp),%rax
  15233e:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
  152342:	48 8d 14 c6                                     	lea    (%rsi,%rax,8),%rdx
  152346:	48 8d 9c 24 d8 01 00 00                         	lea    0x1d8(%rsp),%rbx
  15234e:	48 89 df                                        	mov    %rbx,%rdi
  152351:	e8 2a 00 02 00                                  	call   172380 <<alloc::vec::Vec<&[i32]> as alloc::vec::spec_from_iter_nested::SpecFromIterNested<&[i32], core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::vec::Vec<i32>>, <alloc::vec::Vec<i32>>::as_slice>>>::from_iter>
  152356:	4b 8d 04 b6                                     	lea    (%r14,%r14,4),%rax
  15235a:	48 8d 04 c5 00 00 00 00                         	lea    0x0(,%rax,8),%rax
  152362:	48 01 e8                                        	add    %rbp,%rax
  152365:	48 89 ac 24 40 02 00 00                         	mov    %rbp,0x240(%rsp)
  15236d:	48 89 84 24 48 02 00 00                         	mov    %rax,0x248(%rsp)
  152375:	4c 89 bc 24 50 02 00 00                         	mov    %r15,0x250(%rsp)
  15237d:	48 89 9c 24 58 02 00 00                         	mov    %rbx,0x258(%rsp)
  152385:	48 8d bc 24 10 01 00 00                         	lea    0x110(%rsp),%rdi
  15238d:	48 8d b4 24 40 02 00 00                         	lea    0x240(%rsp),%rsi
  152395:	e8 f6 a2 f4 ff                                  	call   9c690 <core::iter::adapters::try_process::<core::iter::adapters::map::Map<core::slice::iter::Iter<emuella_j2k_codestream::DecompSubbandSpec>, emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl<false, false>::{closure#6}>, u8, core::result::Result<core::convert::Infallible, emuella_j2k_codestream::CodestreamError>, <core::result::Result<alloc::vec::Vec<u8>, emuella_j2k_codestream::CodestreamError> as core::iter::traits::collect::FromIterator<core::result::Result<u8, emuella_j2k_codestream::CodestreamError>>>::from_iter<core::iter::adapters::map::Map<core::slice::iter::Iter<emuella_j2k_codestream::DecompSubbandSpec>, emuella_j2k_codestream::scalable_lossless::encode_lossless_d2_impl<false, false>::{closure#6}>>::{closure#0}, alloc::vec::Vec<u8>>>
  15239a:	48 8b 84 24 10 01 00 00                         	mov    0x110(%rsp),%rax
  1523a2:	48 8b b4 24 18 01 00 00                         	mov    0x118(%rsp),%rsi
  1523aa:	48 8b 94 24 20 01 00 00                         	mov    0x120(%rsp),%rdx
  1523b2:	4c 8b bc 24 28 01 00 00                         	mov    0x128(%rsp),%r15
  1523ba:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
  1523be:	48 8b 8c 24 b8 04 00 00                         	mov    0x4b8(%rsp),%rcx
  1523c6:	48 8b 5c 24 40                                  	mov    0x40(%rsp),%rbx
  1523cb:	74 28                                           	je     1523f5 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x9a5>
  1523cd:	f3 0f 6f 84 24 30 01 00 00                      	movdqu 0x130(%rsp),%xmm0
  1523d6:	f3 41 0f 7f 44 24 20                            	movdqu %xmm0,0x20(%r12)
  1523dd:	49 89 04 24                                     	mov    %rax,(%r12)
  1523e1:	49 89 74 24 08                                  	mov    %rsi,0x8(%r12)
  1523e6:	49 89 54 24 10                                  	mov    %rdx,0x10(%r12)
  1523eb:	4d 89 7c 24 18                                  	mov    %r15,0x18(%r12)
  1523f0:	e9 b6 09 00 00                                  	jmp    152dab <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x135b>
  1523f5:	48 89 74 24 18                                  	mov    %rsi,0x18(%rsp)
  1523fa:	48 89 54 24 30                                  	mov    %rdx,0x30(%rsp)
  1523ff:	48 c7 44 24 48 00 00 00 00                      	movq   $0x0,0x48(%rsp)
  152408:	48 c7 44 24 50 01 00 00 00                      	movq   $0x1,0x50(%rsp)
  152411:	48 c7 44 24 58 00 00 00 00                      	movq   $0x0,0x58(%rsp)
  15241a:	48 8d bc 24 10 01 00 00                         	lea    0x110(%rsp),%rdi
  152422:	48 8d 74 24 48                                  	lea    0x48(%rsp),%rsi
  152427:	ba 80 00 00 00                                  	mov    $0x80,%edx
  15242c:	e8 8f f0 ff ff                                  	call   1514c0 <emuella_j2k_codestream::scalable_lossless::reserve_output>
  152431:	48 83 bc 24 10 01 00 00 ff                      	cmpq   $0xffffffffffffffff,0x110(%rsp)
  15243a:	74 34                                           	je     152470 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xa20>
  15243c:	f3 0f 6f 84 24 10 01 00 00                      	movdqu 0x110(%rsp),%xmm0
  152445:	f3 0f 6f 8c 24 20 01 00 00                      	movdqu 0x120(%rsp),%xmm1
  15244e:	f3 0f 6f 94 24 30 01 00 00                      	movdqu 0x130(%rsp),%xmm2
  152457:	f3 41 0f 7f 54 24 20                            	movdqu %xmm2,0x20(%r12)
  15245e:	f3 41 0f 7f 4c 24 10                            	movdqu %xmm1,0x10(%r12)
  152465:	f3 41 0f 7f 04 24                               	movdqu %xmm0,(%r12)
  15246b:	e9 0e 09 00 00                                  	jmp    152d7e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x132e>
  152470:	48 89 6c 24 08                                  	mov    %rbp,0x8(%rsp)
  152475:	31 c0                                           	xor    %eax,%eax
  152477:	4c 8b 94 24 88 00 00 00                         	mov    0x88(%rsp),%r10
  15247f:	49 83 fa 03                                     	cmp    $0x3,%r10
  152483:	0f 94 c0                                        	sete   %al
  152486:	8b 54 24 14                                     	mov    0x14(%rsp),%edx
  15248a:	8b 4c 24 3c                                     	mov    0x3c(%rsp),%ecx
  15248e:	48 8d bc 24 10 01 00 00                         	lea    0x110(%rsp),%rdi
  152496:	48 8d 74 24 48                                  	lea    0x48(%rsp),%rsi
  15249b:	41 89 d0                                        	mov    %edx,%r8d
  15249e:	41 89 c9                                        	mov    %ecx,%r9d
  1524a1:	6a 00                                           	push   $0x0
  1524a3:	41 57                                           	push   %r15
  1524a5:	ff 74 24 40                                     	push   0x40(%rsp)
  1524a9:	50                                              	push   %rax
  1524aa:	41 52                                           	push   %r10
  1524ac:	ff b4 24 48 02 00 00                            	push   0x248(%rsp)
  1524b3:	e8 f8 4f f8 ff                                  	call   d74b0 <emuella_j2k_codestream::write_native_main_header>
  1524b8:	48 83 c4 30                                     	add    $0x30,%rsp
  1524bc:	48 83 bc 24 10 01 00 00 ff                      	cmpq   $0xffffffffffffffff,0x110(%rsp)
  1524c5:	74 38                                           	je     1524ff <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xaaf>
  1524c7:	f3 0f 6f 84 24 10 01 00 00                      	movdqu 0x110(%rsp),%xmm0
  1524d0:	f3 0f 6f 8c 24 20 01 00 00                      	movdqu 0x120(%rsp),%xmm1
  1524d9:	f3 0f 6f 94 24 30 01 00 00                      	movdqu 0x130(%rsp),%xmm2
  1524e2:	4c 8b 24 24                                     	mov    (%rsp),%r12
  1524e6:	f3 41 0f 7f 54 24 20                            	movdqu %xmm2,0x20(%r12)
  1524ed:	f3 41 0f 7f 4c 24 10                            	movdqu %xmm1,0x10(%r12)
  1524f4:	f3 41 0f 7f 04 24                               	movdqu %xmm0,(%r12)
  1524fa:	e9 75 08 00 00                                  	jmp    152d74 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1324>
  1524ff:	48 8b 44 24 58                                  	mov    0x58(%rsp),%rax
  152504:	48 89 84 24 98 00 00 00                         	mov    %rax,0x98(%rsp)
  15250c:	48 8d 35 b4 16 ec ff                            	lea    -0x13e94c(%rip),%rsi        # 13bc7 <crossbeam_epoch::guard::unprotected::UNPROTECTED+0xe67>
  152513:	48 8d 7c 24 48                                  	lea    0x48(%rsp),%rdi
  152518:	ba 0e 00 00 00                                  	mov    $0xe,%edx
  15251d:	e8 fe b6 ff ff                                  	call   14dc20 <<alloc::vec::Vec<u8>>::append_elements>
  152522:	48 c7 84 24 10 01 00 00 00 00 00 00             	movq   $0x0,0x110(%rsp)
  15252e:	48 c7 84 24 18 01 00 00 02 00 00 00             	movq   $0x2,0x118(%rsp)
  15253a:	66 0f ef c0                                     	pxor   %xmm0,%xmm0
  15253e:	f3 0f 7f 84 24 20 01 00 00                      	movdqu %xmm0,0x120(%rsp)
  152547:	48 c7 84 24 30 01 00 00 01 00 00 00             	movq   $0x1,0x130(%rsp)
  152553:	f3 0f 7f 84 24 38 01 00 00                      	movdqu %xmm0,0x138(%rsp)
  15255c:	48 c7 84 24 48 01 00 00 04 00 00 00             	movq   $0x4,0x148(%rsp)
  152568:	f3 0f 7f 84 24 50 01 00 00                      	movdqu %xmm0,0x150(%rsp)
  152571:	48 c7 84 24 60 01 00 00 08 00 00 00             	movq   $0x8,0x160(%rsp)
  15257d:	f3 0f 7f 84 24 68 01 00 00                      	movdqu %xmm0,0x168(%rsp)
  152586:	48 c7 84 24 78 01 00 00 01 00 00 00             	movq   $0x1,0x178(%rsp)
  152592:	f3 0f 7f 84 24 80 01 00 00                      	movdqu %xmm0,0x180(%rsp)
  15259b:	48 c7 84 24 90 01 00 00 01 00 00 00             	movq   $0x1,0x190(%rsp)
  1525a7:	f3 0f 7f 84 24 98 01 00 00                      	movdqu %xmm0,0x198(%rsp)
  1525b0:	48 c7 84 24 a8 01 00 00 04 00 00 00             	movq   $0x4,0x1a8(%rsp)
  1525bc:	48 c7 84 24 b0 01 00 00 00 00 00 00             	movq   $0x0,0x1b0(%rsp)
  1525c8:	31 f6                                           	xor    %esi,%esi
  1525ca:	48 8b 84 24 d0 00 00 00                         	mov    0xd0(%rsp),%rax
  1525d2:	48 83 f8 02                                     	cmp    $0x2,%rax
  1525d6:	48 0f 43 f0                                     	cmovae %rax,%rsi
  1525da:	48 8d bc 24 40 02 00 00                         	lea    0x240(%rsp),%rdi
  1525e2:	e8 39 33 ff ff                                  	call   145920 <<emuella_j2k_codestream::scalable_lossless::parallel::BlockWorkers>::new>
  1525e7:	8b 84 24 40 02 00 00                            	mov    0x240(%rsp),%eax
  1525ee:	0f 10 84 24 48 02 00 00                         	movups 0x248(%rsp),%xmm0
  1525f6:	0f 29 84 24 f0 01 00 00                         	movaps %xmm0,0x1f0(%rsp)
  1525fe:	0f 10 84 24 58 02 00 00                         	movups 0x258(%rsp),%xmm0
  152606:	0f 29 84 24 00 02 00 00                         	movaps %xmm0,0x200(%rsp)
  15260e:	0f 10 84 24 68 02 00 00                         	movups 0x268(%rsp),%xmm0
  152616:	0f 29 84 24 10 02 00 00                         	movaps %xmm0,0x210(%rsp)
  15261e:	83 f8 01                                        	cmp    $0x1,%eax
  152621:	75 32                                           	jne    152655 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xc05>
  152623:	66 0f 6f 84 24 f0 01 00 00                      	movdqa 0x1f0(%rsp),%xmm0
  15262c:	66 0f 6f 8c 24 00 02 00 00                      	movdqa 0x200(%rsp),%xmm1
  152635:	66 0f 6f 94 24 10 02 00 00                      	movdqa 0x210(%rsp),%xmm2
  15263e:	48 8b 04 24                                     	mov    (%rsp),%rax
  152642:	f3 0f 7f 50 20                                  	movdqu %xmm2,0x20(%rax)
  152647:	f3 0f 7f 48 10                                  	movdqu %xmm1,0x10(%rax)
  15264c:	f3 0f 7f 00                                     	movdqu %xmm0,(%rax)
  152650:	e9 0e 07 00 00                                  	jmp    152d63 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1313>
  152655:	66 0f 6f 84 24 f0 01 00 00                      	movdqa 0x1f0(%rsp),%xmm0
  15265e:	66 0f 6f 8c 24 00 02 00 00                      	movdqa 0x200(%rsp),%xmm1
  152667:	66 0f 6f 94 24 10 02 00 00                      	movdqa 0x210(%rsp),%xmm2
  152670:	66 0f 7f 94 24 40 03 00 00                      	movdqa %xmm2,0x340(%rsp)
  152679:	66 0f 7f 8c 24 30 03 00 00                      	movdqa %xmm1,0x330(%rsp)
  152682:	66 0f 7f 84 24 20 03 00 00                      	movdqa %xmm0,0x320(%rsp)
  15268b:	4d 39 f7                                        	cmp    %r14,%r15
  15268e:	4d 0f 42 f7                                     	cmovb  %r15,%r14
  152692:	48 8b 84 24 80 00 00 00                         	mov    0x80(%rsp),%rax
  15269a:	48 85 c0                                        	test   %rax,%rax
  15269d:	0f 84 49 05 00 00                               	je     152bec <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x119c>
  1526a3:	48 8b 8c 24 28 03 00 00                         	mov    0x328(%rsp),%rcx
  1526ab:	48 89 4c 24 68                                  	mov    %rcx,0x68(%rsp)
  1526b0:	48 8b 8c 24 30 03 00 00                         	mov    0x330(%rsp),%rcx
  1526b8:	48 89 8c 24 d8 00 00 00                         	mov    %rcx,0xd8(%rsp)
  1526c0:	48 c7 44 24 20 00 00 00 00                      	movq   $0x0,0x20(%rsp)
  1526c9:	48 8b 54 24 20                                  	mov    0x20(%rsp),%rdx
  1526ce:	8d 4a 01                                        	lea    0x1(%rdx),%ecx
  1526d1:	80 fa 02                                        	cmp    $0x2,%dl
  1526d4:	0f b6 d1                                        	movzbl %cl,%edx
  1526d7:	b9 02 00 00 00                                  	mov    $0x2,%ecx
  1526dc:	0f 44 d1                                        	cmove  %ecx,%edx
  1526df:	89 94 24 90 00 00 00                            	mov    %edx,0x90(%rsp)
  1526e6:	48 85 c0                                        	test   %rax,%rax
  1526e9:	0f 84 d3 04 00 00                               	je     152bc2 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1172>
  1526ef:	48 8b 4c 24 78                                  	mov    0x78(%rsp),%rcx
  1526f4:	48 8d 04 40                                     	lea    (%rax,%rax,2),%rax
  1526f8:	48 89 4c 24 28                                  	mov    %rcx,0x28(%rsp)
  1526fd:	48 8d 04 c1                                     	lea    (%rcx,%rax,8),%rax
  152701:	48 89 84 24 88 00 00 00                         	mov    %rax,0x88(%rsp)
  152709:	48 8b 44 24 58                                  	mov    0x58(%rsp),%rax
  15270e:	48 89 44 24 60                                  	mov    %rax,0x60(%rsp)
  152713:	bf 90 00 00 00                                  	mov    $0x90,%edi
  152718:	ff 15 9a 26 12 00                               	call   *0x12269a(%rip)        # 274db8 <malloc@GLIBC_2.2.5>
  15271e:	48 85 c0                                        	test   %rax,%rax
  152721:	0f 84 78 07 00 00                               	je     152e9f <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x144f>
  152727:	48 c7 84 24 f8 00 00 00 03 00 00 00             	movq   $0x3,0xf8(%rsp)
  152733:	48 89 84 24 00 01 00 00                         	mov    %rax,0x100(%rsp)
  15273b:	48 c7 84 24 08 01 00 00 00 00 00 00             	movq   $0x0,0x108(%rsp)
  152747:	48 c7 84 24 28 02 00 00 00 00 00 00             	movq   $0x0,0x228(%rsp)
  152753:	48 c7 84 24 30 02 00 00 08 00 00 00             	movq   $0x8,0x230(%rsp)
  15275f:	48 c7 84 24 38 02 00 00 00 00 00 00             	movq   $0x0,0x238(%rsp)
  15276b:	48 83 bc 24 d0 00 00 00 01                      	cmpq   $0x1,0xd0(%rsp)
  152774:	0f 86 56 01 00 00                               	jbe    1528d0 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xe80>
  15277a:	4c 8b 64 24 08                                  	mov    0x8(%rsp),%r12
  15277f:	31 db                                           	xor    %ebx,%ebx
  152781:	48 8b 6c 24 20                                  	mov    0x20(%rsp),%rbp
  152786:	eb 07                                           	jmp    15278f <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xd3f>
  152788:	49 83 c4 28                                     	add    $0x28,%r12
  15278c:	48 ff c3                                        	inc    %rbx
  15278f:	4c 39 f3                                        	cmp    %r14,%rbx
  152792:	0f 83 2b 02 00 00                               	jae    1529c3 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xf73>
  152798:	41 38 6c 24 21                                  	cmp    %bpl,0x21(%r12)
  15279d:	75 e9                                           	jne    152788 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xd38>
  15279f:	48 8b 44 24 28                                  	mov    0x28(%rsp),%rax
  1527a4:	4c 8b 40 08                                     	mov    0x8(%rax),%r8
  1527a8:	4c 8b 48 10                                     	mov    0x10(%rax),%r9
  1527ac:	49 8b 44 24 20                                  	mov    0x20(%r12),%rax
  1527b1:	48 89 84 24 c0 00 00 00                         	mov    %rax,0xc0(%rsp)
  1527b9:	f3 41 0f 6f 04 24                               	movdqu (%r12),%xmm0
  1527bf:	f3 41 0f 6f 4c 24 10                            	movdqu 0x10(%r12),%xmm1
  1527c6:	66 0f 7f 8c 24 b0 00 00 00                      	movdqa %xmm1,0xb0(%rsp)
  1527cf:	66 0f 7f 84 24 a0 00 00 00                      	movdqa %xmm0,0xa0(%rsp)
  1527d8:	8b 4c 24 14                                     	mov    0x14(%rsp),%ecx
  1527dc:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  1527e1:	0f b6 04 18                                     	movzbl (%rax,%rbx,1),%eax
  1527e5:	48 8d bc 24 40 02 00 00                         	lea    0x240(%rsp),%rdi
  1527ed:	48 8b 74 24 68                                  	mov    0x68(%rsp),%rsi
  1527f2:	48 8b 94 24 d8 00 00 00                         	mov    0xd8(%rsp),%rdx
  1527fa:	ff b4 24 b8 04 00 00                            	push   0x4b8(%rsp)
  152801:	4c 8d 54 24 50                                  	lea    0x50(%rsp),%r10
  152806:	41 52                                           	push   %r10
  152808:	50                                              	push   %rax
  152809:	48 8d 84 24 b8 00 00 00                         	lea    0xb8(%rsp),%rax
  152811:	50                                              	push   %rax
  152812:	e8 b9 4e f3 ff                                  	call   876d0 <<emuella_j2k_codestream::scalable_lossless::parallel::BlockWorkers>::encode_subband::<false, false>>
  152817:	48 83 c4 20                                     	add    $0x20,%rsp
  15281b:	48 8b 84 24 40 02 00 00                         	mov    0x240(%rsp),%rax
  152823:	48 8b ac 24 70 02 00 00                         	mov    0x270(%rsp),%rbp
  15282b:	48 8d 8c 24 48 02 00 00                         	lea    0x248(%rsp),%rcx
  152833:	0f 10 01                                        	movups (%rcx),%xmm0
  152836:	0f 10 49 10                                     	movups 0x10(%rcx),%xmm1
  15283a:	0f 29 84 24 f0 02 00 00                         	movaps %xmm0,0x2f0(%rsp)
  152842:	0f 29 8c 24 00 03 00 00                         	movaps %xmm1,0x300(%rsp)
  15284a:	48 8b 49 20                                     	mov    0x20(%rcx),%rcx
  15284e:	48 89 8c 24 10 03 00 00                         	mov    %rcx,0x310(%rsp)
  152856:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
  15285a:	0f 84 b1 04 00 00                               	je     152d11 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x12c1>
  152860:	4c 8b bc 24 78 02 00 00                         	mov    0x278(%rsp),%r15
  152868:	48 89 84 24 f0 01 00 00                         	mov    %rax,0x1f0(%rsp)
  152870:	48 8b 84 24 10 03 00 00                         	mov    0x310(%rsp),%rax
  152878:	48 8d 8c 24 f8 01 00 00                         	lea    0x1f8(%rsp),%rcx
  152880:	48 89 41 20                                     	mov    %rax,0x20(%rcx)
  152884:	66 0f 6f 84 24 f0 02 00 00                      	movdqa 0x2f0(%rsp),%xmm0
  15288d:	66 0f 6f 8c 24 00 03 00 00                      	movdqa 0x300(%rsp),%xmm1
  152896:	f3 0f 7f 49 10                                  	movdqu %xmm1,0x10(%rcx)
  15289b:	f3 0f 7f 01                                     	movdqu %xmm0,(%rcx)
  15289f:	48 8d bc 24 f8 00 00 00                         	lea    0xf8(%rsp),%rdi
  1528a7:	48 8d b4 24 f0 01 00 00                         	lea    0x1f0(%rsp),%rsi
  1528af:	e8 5c 8f ff ff                                  	call   14b810 <<alloc::vec::Vec<emuella_j2k_codestream::NativeDecompSubband>>::push_mut>
  1528b4:	48 85 ed                                        	test   %rbp,%rbp
  1528b7:	48 8b 6c 24 20                                  	mov    0x20(%rsp),%rbp
  1528bc:	0f 84 c6 fe ff ff                               	je     152788 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xd38>
  1528c2:	4c 89 ff                                        	mov    %r15,%rdi
  1528c5:	ff 15 fd 24 12 00                               	call   *0x1224fd(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  1528cb:	e9 b8 fe ff ff                                  	jmp    152788 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xd38>
  1528d0:	48 8b 5c 24 08                                  	mov    0x8(%rsp),%rbx
  1528d5:	45 31 ff                                        	xor    %r15d,%r15d
  1528d8:	4c 8b 64 24 20                                  	mov    0x20(%rsp),%r12
  1528dd:	eb 07                                           	jmp    1528e6 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xe96>
  1528df:	48 83 c3 28                                     	add    $0x28,%rbx
  1528e3:	49 ff c7                                        	inc    %r15
  1528e6:	4d 39 f7                                        	cmp    %r14,%r15
  1528e9:	0f 83 d4 00 00 00                               	jae    1529c3 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xf73>
  1528ef:	44 38 63 21                                     	cmp    %r12b,0x21(%rbx)
  1528f3:	75 ea                                           	jne    1528df <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xe8f>
  1528f5:	48 8b 44 24 28                                  	mov    0x28(%rsp),%rax
  1528fa:	48 8b 50 08                                     	mov    0x8(%rax),%rdx
  1528fe:	48 8b 48 10                                     	mov    0x10(%rax),%rcx
  152902:	48 8b 43 20                                     	mov    0x20(%rbx),%rax
  152906:	48 89 84 24 c0 00 00 00                         	mov    %rax,0xc0(%rsp)
  15290e:	f3 0f 6f 03                                     	movdqu (%rbx),%xmm0
  152912:	f3 0f 6f 4b 10                                  	movdqu 0x10(%rbx),%xmm1
  152917:	66 0f 7f 8c 24 b0 00 00 00                      	movdqa %xmm1,0xb0(%rsp)
  152920:	66 0f 7f 84 24 a0 00 00 00                      	movdqa %xmm0,0xa0(%rsp)
  152929:	8b 74 24 14                                     	mov    0x14(%rsp),%esi
  15292d:	48 8b 44 24 30                                  	mov    0x30(%rsp),%rax
  152932:	46 0f b6 0c 38                                  	movzbl (%rax,%r15,1),%r9d
  152937:	48 8d bc 24 40 02 00 00                         	lea    0x240(%rsp),%rdi
  15293f:	4c 8d 84 24 a0 00 00 00                         	lea    0xa0(%rsp),%r8
  152947:	ff b4 24 b8 04 00 00                            	push   0x4b8(%rsp)
  15294e:	6a 01                                           	push   $0x1
  152950:	48 8d 84 24 20 01 00 00                         	lea    0x120(%rsp),%rax
  152958:	50                                              	push   %rax
  152959:	48 8d 44 24 60                                  	lea    0x60(%rsp),%rax
  15295e:	50                                              	push   %rax
  15295f:	e8 7c 3c f3 ff                                  	call   865e0 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>>
  152964:	48 83 c4 20                                     	add    $0x20,%rsp
  152968:	8b 84 24 40 02 00 00                            	mov    0x240(%rsp),%eax
  15296f:	48 8d 8c 24 48 02 00 00                         	lea    0x248(%rsp),%rcx
  152977:	f3 0f 6f 01                                     	movdqu (%rcx),%xmm0
  15297b:	f3 0f 6f 49 10                                  	movdqu 0x10(%rcx),%xmm1
  152980:	f3 0f 6f 51 20                                  	movdqu 0x20(%rcx),%xmm2
  152985:	66 0f 7f 84 24 80 03 00 00                      	movdqa %xmm0,0x380(%rsp)
  15298e:	66 0f 7f 8c 24 90 03 00 00                      	movdqa %xmm1,0x390(%rsp)
  152997:	66 0f 7f 94 24 a0 03 00 00                      	movdqa %xmm2,0x3a0(%rsp)
  1529a0:	83 f8 01                                        	cmp    $0x1,%eax
  1529a3:	0f 84 3d 03 00 00                               	je     152ce6 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1296>
  1529a9:	48 8d bc 24 f8 00 00 00                         	lea    0xf8(%rsp),%rdi
  1529b1:	48 8d b4 24 80 03 00 00                         	lea    0x380(%rsp),%rsi
  1529b9:	e8 52 8e ff ff                                  	call   14b810 <<alloc::vec::Vec<emuella_j2k_codestream::NativeDecompSubband>>::push_mut>
  1529be:	e9 1c ff ff ff                                  	jmp    1528df <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xe8f>
  1529c3:	48 c7 84 24 a0 00 00 00 00 00 00 00             	movq   $0x0,0xa0(%rsp)
  1529cf:	48 c7 84 24 a8 00 00 00 01 00 00 00             	movq   $0x1,0xa8(%rsp)
  1529db:	48 8d 84 24 b0 00 00 00                         	lea    0xb0(%rsp),%rax
  1529e3:	66 0f ef c0                                     	pxor   %xmm0,%xmm0
  1529e7:	f3 0f 7f 00                                     	movdqu %xmm0,(%rax)
  1529eb:	4c 8b bc 24 00 01 00 00                         	mov    0x100(%rsp),%r15
  1529f3:	48 8b 9c 24 08 01 00 00                         	mov    0x108(%rsp),%rbx
  1529fb:	48 89 d8                                        	mov    %rbx,%rax
  1529fe:	48 c1 e0 04                                     	shl    $0x4,%rax
  152a02:	4c 8d 24 40                                     	lea    (%rax,%rax,2),%r12
  152a06:	48 85 db                                        	test   %rbx,%rbx
  152a09:	74 31                                           	je     152a3c <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xfec>
  152a0b:	4b 8d 04 27                                     	lea    (%r15,%r12,1),%rax
  152a0f:	4c 89 f9                                        	mov    %r15,%rcx
  152a12:	48 8b 71 08                                     	mov    0x8(%rcx),%rsi
  152a16:	48 8b 51 10                                     	mov    0x10(%rcx),%rdx
  152a1a:	48 83 c1 30                                     	add    $0x30,%rcx
  152a1e:	48 c1 e2 05                                     	shl    $0x5,%rdx
  152a22:	48 85 d2                                        	test   %rdx,%rdx
  152a25:	74 10                                           	je     152a37 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xfe7>
  152a27:	48 83 c2 e0                                     	add    $0xffffffffffffffe0,%rdx
  152a2b:	80 7e 1b 00                                     	cmpb   $0x0,0x1b(%rsi)
  152a2f:	48 8d 76 20                                     	lea    0x20(%rsi),%rsi
  152a33:	74 ed                                           	je     152a22 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xfd2>
  152a35:	eb 09                                           	jmp    152a40 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xff0>
  152a37:	48 39 c1                                        	cmp    %rax,%rcx
  152a3a:	75 d6                                           	jne    152a12 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xfc2>
  152a3c:	31 ed                                           	xor    %ebp,%ebp
  152a3e:	eb 03                                           	jmp    152a43 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xff3>
  152a40:	40 b5 01                                        	mov    $0x1,%bpl
  152a43:	40 0f b6 d5                                     	movzbl %bpl,%edx
  152a47:	48 8d bc 24 40 02 00 00                         	lea    0x240(%rsp),%rdi
  152a4f:	48 8d b4 24 a0 00 00 00                         	lea    0xa0(%rsp),%rsi
  152a57:	e8 64 a3 ff ff                                  	call   14cdc0 <<emuella_j2k_codestream::PacketBitWriter>::write_bit>
  152a5c:	48 83 bc 24 40 02 00 00 ff                      	cmpq   $0xffffffffffffffff,0x240(%rsp)
  152a65:	0f 85 b0 01 00 00                               	jne    152c1b <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x11cb>
  152a6b:	48 85 db                                        	test   %rbx,%rbx
  152a6e:	0f 94 c0                                        	sete   %al
  152a71:	40 80 f5 01                                     	xor    $0x1,%bpl
  152a75:	40 08 c5                                        	or     %al,%bpl
  152a78:	75 44                                           	jne    152abe <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x106e>
  152a7a:	31 db                                           	xor    %ebx,%ebx
  152a7c:	4d 8b 44 1f 08                                  	mov    0x8(%r15,%rbx,1),%r8
  152a81:	4d 8b 4c 1f 10                                  	mov    0x10(%r15,%rbx,1),%r9
  152a86:	41 0f b7 4c 1f 2a                               	movzwl 0x2a(%r15,%rbx,1),%ecx
  152a8c:	41 8b 54 1f 28                                  	mov    0x28(%r15,%rbx,1),%edx
  152a91:	48 8d bc 24 40 02 00 00                         	lea    0x240(%rsp),%rdi
  152a99:	48 8d b4 24 a0 00 00 00                         	lea    0xa0(%rsp),%rsi
  152aa1:	e8 aa 27 f9 ff                                  	call   e5250 <emuella_j2k_codestream::write_component_packet_header>
  152aa6:	48 83 bc 24 40 02 00 00 ff                      	cmpq   $0xffffffffffffffff,0x240(%rsp)
  152aaf:	0f 85 66 01 00 00                               	jne    152c1b <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x11cb>
  152ab5:	48 83 c3 30                                     	add    $0x30,%rbx
  152ab9:	49 39 dc                                        	cmp    %rbx,%r12
  152abc:	75 be                                           	jne    152a7c <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x102c>
  152abe:	48 8b 84 24 b8 00 00 00                         	mov    0xb8(%rsp),%rax
  152ac6:	a8 07                                           	test   $0x7,%al
  152ac8:	74 10                                           	je     152ada <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x108a>
  152aca:	48 83 e0 f8                                     	and    $0xfffffffffffffff8,%rax
  152ace:	48 83 c0 08                                     	add    $0x8,%rax
  152ad2:	48 89 84 24 b8 00 00 00                         	mov    %rax,0xb8(%rsp)
  152ada:	48 8b 9c 24 a8 00 00 00                         	mov    0xa8(%rsp),%rbx
  152ae2:	4c 8b a4 24 b0 00 00 00                         	mov    0xb0(%rsp),%r12
  152aea:	48 8d bc 24 40 02 00 00                         	lea    0x240(%rsp),%rdi
  152af2:	48 8d 74 24 48                                  	lea    0x48(%rsp),%rsi
  152af7:	4c 89 e2                                        	mov    %r12,%rdx
  152afa:	48 8b 8c 24 b8 04 00 00                         	mov    0x4b8(%rsp),%rcx
  152b02:	e8 b9 e9 ff ff                                  	call   1514c0 <emuella_j2k_codestream::scalable_lossless::reserve_output>
  152b07:	48 83 bc 24 40 02 00 00 ff                      	cmpq   $0xffffffffffffffff,0x240(%rsp)
  152b10:	0f 85 05 01 00 00                               	jne    152c1b <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x11cb>
  152b16:	48 8b 6c 24 58                                  	mov    0x58(%rsp),%rbp
  152b1b:	49 8d 34 2c                                     	lea    (%r12,%rbp,1),%rsi
  152b1f:	48 8d 7c 24 48                                  	lea    0x48(%rsp),%rdi
  152b24:	e8 07 34 ff ff                                  	call   145f30 <<alloc::vec::Vec<u8>>::resize>
  152b29:	48 8b 7c 24 50                                  	mov    0x50(%rsp),%rdi
  152b2e:	48 8b 74 24 58                                  	mov    0x58(%rsp),%rsi
  152b33:	48 8b 54 24 60                                  	mov    0x60(%rsp),%rdx
  152b38:	4d 8d 3c 14                                     	lea    (%r12,%rdx,1),%r15
  152b3c:	48 89 e9                                        	mov    %rbp,%rcx
  152b3f:	4d 89 f8                                        	mov    %r15,%r8
  152b42:	e8 99 4a f3 ff                                  	call   875e0 <<[u8]>::copy_within::<core::ops::range::Range<usize>>>
  152b47:	48 8b 54 24 58                                  	mov    0x58(%rsp),%rdx
  152b4c:	4c 3b 7c 24 60                                  	cmp    0x60(%rsp),%r15
  152b51:	0f 82 1f 03 00 00                               	jb     152e76 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1426>
  152b57:	49 39 d7                                        	cmp    %rdx,%r15
  152b5a:	0f 87 16 03 00 00                               	ja     152e76 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1426>
  152b60:	48 8b 7c 24 60                                  	mov    0x60(%rsp),%rdi
  152b65:	48 03 7c 24 50                                  	add    0x50(%rsp),%rdi
  152b6a:	48 89 de                                        	mov    %rbx,%rsi
  152b6d:	4c 89 e2                                        	mov    %r12,%rdx
  152b70:	41 ff d5                                        	call   *%r13
  152b73:	48 83 bc 24 a0 00 00 00 00                      	cmpq   $0x0,0xa0(%rsp)
  152b7c:	74 0e                                           	je     152b8c <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x113c>
  152b7e:	48 8b bc 24 a8 00 00 00                         	mov    0xa8(%rsp),%rdi
  152b86:	ff 15 3c 22 12 00                               	call   *0x12223c(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152b8c:	48 8b 5c 24 28                                  	mov    0x28(%rsp),%rbx
  152b91:	48 83 c3 18                                     	add    $0x18,%rbx
  152b95:	48 8d bc 24 28 02 00 00                         	lea    0x228(%rsp),%rdi
  152b9d:	e8 be e9 f3 ff                                  	call   91560 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<emuella_j2k_accel::OutputRect>>>>
  152ba2:	48 8d bc 24 f8 00 00 00                         	lea    0xf8(%rsp),%rdi
  152baa:	e8 81 f0 f3 ff                                  	call   91c30 <core::ptr::drop_glue::<alloc::vec::Vec<emuella_j2k_codestream::NativeDecompSubband>>>
  152baf:	48 89 5c 24 28                                  	mov    %rbx,0x28(%rsp)
  152bb4:	48 3b 9c 24 88 00 00 00                         	cmp    0x88(%rsp),%rbx
  152bbc:	0f 85 47 fb ff ff                               	jne    152709 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xcb9>
  152bc2:	80 7c 24 20 02                                  	cmpb   $0x2,0x20(%rsp)
  152bc7:	74 23                                           	je     152bec <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x119c>
  152bc9:	80 bc 24 90 00 00 00 02                         	cmpb   $0x2,0x90(%rsp)
  152bd1:	77 19                                           	ja     152bec <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x119c>
  152bd3:	48 8b 84 24 80 00 00 00                         	mov    0x80(%rsp),%rax
  152bdb:	8b 8c 24 90 00 00 00                            	mov    0x90(%rsp),%ecx
  152be2:	48 89 4c 24 20                                  	mov    %rcx,0x20(%rsp)
  152be7:	e9 dd fa ff ff                                  	jmp    1526c9 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0xc79>
  152bec:	48 8b 54 24 58                                  	mov    0x58(%rsp),%rdx
  152bf1:	48 89 d0                                        	mov    %rdx,%rax
  152bf4:	48 2b 84 24 98 00 00 00                         	sub    0x98(%rsp),%rax
  152bfc:	48 89 c1                                        	mov    %rax,%rcx
  152bff:	48 c1 e9 20                                     	shr    $0x20,%rcx
  152c03:	74 61                                           	je     152c66 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1216>
  152c05:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
  152c0f:	48 8b 0c 24                                     	mov    (%rsp),%rcx
  152c13:	48 89 01                                        	mov    %rax,(%rcx)
  152c16:	e9 3b 01 00 00                                  	jmp    152d56 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1306>
  152c1b:	0f 10 84 24 40 02 00 00                         	movups 0x240(%rsp),%xmm0
  152c23:	0f 10 8c 24 50 02 00 00                         	movups 0x250(%rsp),%xmm1
  152c2b:	f3 0f 6f 94 24 60 02 00 00                      	movdqu 0x260(%rsp),%xmm2
  152c34:	48 8b 04 24                                     	mov    (%rsp),%rax
  152c38:	f3 0f 7f 50 20                                  	movdqu %xmm2,0x20(%rax)
  152c3d:	0f 11 48 10                                     	movups %xmm1,0x10(%rax)
  152c41:	0f 11 00                                        	movups %xmm0,(%rax)
  152c44:	48 83 bc 24 a0 00 00 00 00                      	cmpq   $0x0,0xa0(%rsp)
  152c4d:	0f 84 e9 00 00 00                               	je     152d3c <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x12ec>
  152c53:	48 8b bc 24 a8 00 00 00                         	mov    0xa8(%rsp),%rdi
  152c5b:	ff 15 67 21 12 00                               	call   *0x122167(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152c61:	e9 d6 00 00 00                                  	jmp    152d3c <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x12ec>
  152c66:	48 8b 8c 24 98 00 00 00                         	mov    0x98(%rsp),%rcx
  152c6e:	48 8d 79 06                                     	lea    0x6(%rcx),%rdi
  152c72:	48 83 c1 0a                                     	add    $0xa,%rcx
  152c76:	48 39 d1                                        	cmp    %rdx,%rcx
  152c79:	0f 87 0e 02 00 00                               	ja     152e8d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x143d>
  152c7f:	48 8b 4c 24 50                                  	mov    0x50(%rsp),%rcx
  152c84:	0f c8                                           	bswap  %eax
  152c86:	89 04 39                                        	mov    %eax,(%rcx,%rdi,1)
  152c89:	48 8d bc 24 40 02 00 00                         	lea    0x240(%rsp),%rdi
  152c91:	48 8d 74 24 48                                  	lea    0x48(%rsp),%rsi
  152c96:	ba 02 00 00 00                                  	mov    $0x2,%edx
  152c9b:	48 8b 8c 24 b8 04 00 00                         	mov    0x4b8(%rsp),%rcx
  152ca3:	e8 18 e8 ff ff                                  	call   1514c0 <emuella_j2k_codestream::scalable_lossless::reserve_output>
  152ca8:	48 83 bc 24 40 02 00 00 ff                      	cmpq   $0xffffffffffffffff,0x240(%rsp)
  152cb1:	0f 84 1e 01 00 00                               	je     152dd5 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1385>
  152cb7:	f3 0f 6f 84 24 40 02 00 00                      	movdqu 0x240(%rsp),%xmm0
  152cc0:	f3 0f 6f 8c 24 50 02 00 00                      	movdqu 0x250(%rsp),%xmm1
  152cc9:	f3 0f 6f 94 24 60 02 00 00                      	movdqu 0x260(%rsp),%xmm2
  152cd2:	48 8b 04 24                                     	mov    (%rsp),%rax
  152cd6:	f3 0f 7f 50 20                                  	movdqu %xmm2,0x20(%rax)
  152cdb:	f3 0f 7f 48 10                                  	movdqu %xmm1,0x10(%rax)
  152ce0:	f3 0f 7f 00                                     	movdqu %xmm0,(%rax)
  152ce4:	eb 70                                           	jmp    152d56 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1306>
  152ce6:	0f 28 84 24 80 03 00 00                         	movaps 0x380(%rsp),%xmm0
  152cee:	0f 28 8c 24 90 03 00 00                         	movaps 0x390(%rsp),%xmm1
  152cf6:	66 0f 6f 94 24 a0 03 00 00                      	movdqa 0x3a0(%rsp),%xmm2
  152cff:	48 8b 04 24                                     	mov    (%rsp),%rax
  152d03:	f3 0f 7f 50 20                                  	movdqu %xmm2,0x20(%rax)
  152d08:	0f 11 48 10                                     	movups %xmm1,0x10(%rax)
  152d0c:	0f 11 00                                        	movups %xmm0,(%rax)
  152d0f:	eb 2b                                           	jmp    152d3c <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x12ec>
  152d11:	48 8b 84 24 10 03 00 00                         	mov    0x310(%rsp),%rax
  152d19:	48 8b 0c 24                                     	mov    (%rsp),%rcx
  152d1d:	48 89 41 20                                     	mov    %rax,0x20(%rcx)
  152d21:	0f 28 84 24 f0 02 00 00                         	movaps 0x2f0(%rsp),%xmm0
  152d29:	0f 28 8c 24 00 03 00 00                         	movaps 0x300(%rsp),%xmm1
  152d31:	0f 11 49 10                                     	movups %xmm1,0x10(%rcx)
  152d35:	0f 11 01                                        	movups %xmm0,(%rcx)
  152d38:	48 89 69 28                                     	mov    %rbp,0x28(%rcx)
  152d3c:	48 8d bc 24 28 02 00 00                         	lea    0x228(%rsp),%rdi
  152d44:	e8 17 e8 f3 ff                                  	call   91560 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<emuella_j2k_accel::OutputRect>>>>
  152d49:	48 8d bc 24 f8 00 00 00                         	lea    0xf8(%rsp),%rdi
  152d51:	e8 da ee f3 ff                                  	call   91c30 <core::ptr::drop_glue::<alloc::vec::Vec<emuella_j2k_codestream::NativeDecompSubband>>>
  152d56:	48 8d bc 24 20 03 00 00                         	lea    0x320(%rsp),%rdi
  152d5e:	e8 cd 7d f4 ff                                  	call   9ab30 <core::ptr::drop_glue::<emuella_j2k_codestream::scalable_lossless::parallel::BlockWorkers>>
  152d63:	48 8d bc 24 10 01 00 00                         	lea    0x110(%rsp),%rdi
  152d6b:	e8 70 63 f4 ff                                  	call   990e0 <core::ptr::drop_glue::<emuella_j2k_tier1::CodeBlockEncodeScratch>>
  152d70:	4c 8b 24 24                                     	mov    (%rsp),%r12
  152d74:	48 8b 6c 24 08                                  	mov    0x8(%rsp),%rbp
  152d79:	48 8b 5c 24 40                                  	mov    0x40(%rsp),%rbx
  152d7e:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
  152d83:	48 83 7c 24 48 00                               	cmpq   $0x0,0x48(%rsp)
  152d89:	74 10                                           	je     152d9b <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x134b>
  152d8b:	48 8b 7c 24 50                                  	mov    0x50(%rsp),%rdi
  152d90:	ff 15 32 20 12 00                               	call   *0x122032(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152d96:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
  152d9b:	48 85 c0                                        	test   %rax,%rax
  152d9e:	48 8b 7c 24 30                                  	mov    0x30(%rsp),%rdi
  152da3:	74 06                                           	je     152dab <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x135b>
  152da5:	ff 15 1d 20 12 00                               	call   *0x12201d(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152dab:	48 83 bc 24 d8 01 00 00 00                      	cmpq   $0x0,0x1d8(%rsp)
  152db4:	74 0e                                           	je     152dc4 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1374>
  152db6:	48 8b bc 24 e0 01 00 00                         	mov    0x1e0(%rsp),%rdi
  152dbe:	ff 15 04 20 12 00                               	call   *0x122004(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152dc4:	48 85 db                                        	test   %rbx,%rbx
  152dc7:	0f 84 50 f5 ff ff                               	je     15231d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x8cd>
  152dcd:	48 89 ef                                        	mov    %rbp,%rdi
  152dd0:	e9 a7 f4 ff ff                                  	jmp    15227c <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x82c>
  152dd5:	48 8d 35 f9 0d ec ff                            	lea    -0x13f207(%rip),%rsi        # 13bd5 <crossbeam_epoch::guard::unprotected::UNPROTECTED+0xe75>
  152ddc:	48 8d 7c 24 48                                  	lea    0x48(%rsp),%rdi
  152de1:	ba 02 00 00 00                                  	mov    $0x2,%edx
  152de6:	e8 35 ae ff ff                                  	call   14dc20 <<alloc::vec::Vec<u8>>::append_elements>
  152deb:	48 8b 44 24 58                                  	mov    0x58(%rsp),%rax
  152df0:	48 8b 0c 24                                     	mov    (%rsp),%rcx
  152df4:	48 89 41 18                                     	mov    %rax,0x18(%rcx)
  152df8:	f3 0f 6f 44 24 48                               	movdqu 0x48(%rsp),%xmm0
  152dfe:	f3 0f 7f 41 08                                  	movdqu %xmm0,0x8(%rcx)
  152e03:	48 c7 01 ff ff ff ff                            	movq   $0xffffffffffffffff,(%rcx)
  152e0a:	48 8d bc 24 20 03 00 00                         	lea    0x320(%rsp),%rdi
  152e12:	e8 19 7d f4 ff                                  	call   9ab30 <core::ptr::drop_glue::<emuella_j2k_codestream::scalable_lossless::parallel::BlockWorkers>>
  152e17:	48 8d bc 24 10 01 00 00                         	lea    0x110(%rsp),%rdi
  152e1f:	e8 bc 62 f4 ff                                  	call   990e0 <core::ptr::drop_glue::<emuella_j2k_tier1::CodeBlockEncodeScratch>>
  152e24:	48 83 7c 24 18 00                               	cmpq   $0x0,0x18(%rsp)
  152e2a:	74 0b                                           	je     152e37 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x13e7>
  152e2c:	48 8b 7c 24 30                                  	mov    0x30(%rsp),%rdi
  152e31:	ff 15 91 1f 12 00                               	call   *0x121f91(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152e37:	48 83 bc 24 d8 01 00 00 00                      	cmpq   $0x0,0x1d8(%rsp)
  152e40:	74 0e                                           	je     152e50 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1400>
  152e42:	48 8b bc 24 e0 01 00 00                         	mov    0x1e0(%rsp),%rdi
  152e4a:	ff 15 78 1f 12 00                               	call   *0x121f78(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152e50:	48 83 7c 24 40 00                               	cmpq   $0x0,0x40(%rsp)
  152e56:	74 0b                                           	je     152e63 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1413>
  152e58:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  152e5d:	ff 15 65 1f 12 00                               	call   *0x121f65(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152e63:	48 8d 7c 24 70                                  	lea    0x70(%rsp),%rdi
  152e68:	e8 f3 e6 f3 ff                                  	call   91560 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<emuella_j2k_accel::OutputRect>>>>
  152e6d:	4c 8b 24 24                                     	mov    (%rsp),%r12
  152e71:	e9 ce ed ff ff                                  	jmp    151c44 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1f4>
  152e76:	48 8d 0d 53 b1 11 00                            	lea    0x11b153(%rip),%rcx        # 26dfd0 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x358>
  152e7d:	48 8b 7c 24 60                                  	mov    0x60(%rsp),%rdi
  152e82:	4c 89 fe                                        	mov    %r15,%rsi
  152e85:	ff 15 95 21 12 00                               	call   *0x122195(%rip)        # 275020 <_DYNAMIC+0x470>
  152e8b:	eb 22                                           	jmp    152eaf <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x145f>
  152e8d:	48 89 ce                                        	mov    %rcx,%rsi
  152e90:	48 8d 0d 09 b1 11 00                            	lea    0x11b109(%rip),%rcx        # 26dfa0 <anon.0090a85a018fa7201b9c60ffeb615359.8.llvm.14346473256741260712+0x328>
  152e97:	ff 15 83 21 12 00                               	call   *0x122183(%rip)        # 275020 <_DYNAMIC+0x470>
  152e9d:	eb 10                                           	jmp    152eaf <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x145f>
  152e9f:	bf 08 00 00 00                                  	mov    $0x8,%edi
  152ea4:	be 90 00 00 00                                  	mov    $0x90,%esi
  152ea9:	ff 15 51 1f 12 00                               	call   *0x121f51(%rip)        # 274e00 <_DYNAMIC+0x250>
  152eaf:	0f 0b                                           	ud2
  152eb1:	e9 9c 00 00 00                                  	jmp    152f52 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1502>
  152eb6:	49 89 c5                                        	mov    %rax,%r13
  152eb9:	48 85 ed                                        	test   %rbp,%rbp
  152ebc:	0f 84 ac 00 00 00                               	je     152f6e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x151e>
  152ec2:	4c 89 ff                                        	mov    %r15,%rdi
  152ec5:	e9 9e 00 00 00                                  	jmp    152f68 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1518>
  152eca:	eb 0d                                           	jmp    152ed9 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1489>
  152ecc:	e9 81 00 00 00                                  	jmp    152f52 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1502>
  152ed1:	49 89 c5                                        	mov    %rax,%r13
  152ed4:	e9 c9 00 00 00                                  	jmp    152fa2 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1552>
  152ed9:	49 89 c5                                        	mov    %rax,%r13
  152edc:	e9 8d 00 00 00                                  	jmp    152f6e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x151e>
  152ee1:	48 89 6c 24 08                                  	mov    %rbp,0x8(%rsp)
  152ee6:	49 89 c5                                        	mov    %rax,%r13
  152ee9:	e9 da 00 00 00                                  	jmp    152fc8 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1578>
  152eee:	48 89 6c 24 08                                  	mov    %rbp,0x8(%rsp)
  152ef3:	49 89 c5                                        	mov    %rax,%r13
  152ef6:	e9 e6 00 00 00                                  	jmp    152fe1 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1591>
  152efb:	eb 40                                           	jmp    152f3d <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x14ed>
  152efd:	49 89 c5                                        	mov    %rax,%r13
  152f00:	e9 83 00 00 00                                  	jmp    152f88 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1538>
  152f05:	49 89 c5                                        	mov    %rax,%r13
  152f08:	48 83 bc 24 e0 00 00 00 00                      	cmpq   $0x0,0xe0(%rsp)
  152f11:	74 0e                                           	je     152f21 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x14d1>
  152f13:	48 8b bc 24 e8 00 00 00                         	mov    0xe8(%rsp),%rdi
  152f1b:	ff 15 a7 1e 12 00                               	call   *0x121ea7(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152f21:	48 83 bc 24 60 04 00 00 00                      	cmpq   $0x0,0x460(%rsp)
  152f2a:	0f 8e 59 f1 ff ff                               	jle    152089 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x639>
  152f30:	48 8b bc 24 68 04 00 00                         	mov    0x468(%rsp),%rdi
  152f38:	e9 46 f1 ff ff                                  	jmp    152083 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x633>
  152f3d:	49 89 c5                                        	mov    %rax,%r13
  152f40:	48 8d 7c 24 70                                  	lea    0x70(%rsp),%rdi
  152f45:	e8 16 e6 f3 ff                                  	call   91560 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<emuella_j2k_accel::OutputRect>>>>
  152f4a:	4c 89 ef                                        	mov    %r13,%rdi
  152f4d:	e8 4e 92 11 00                                  	call   26c1a0 <_Unwind_Resume@plt>
  152f52:	49 89 c5                                        	mov    %rax,%r13
  152f55:	48 83 bc 24 a0 00 00 00 00                      	cmpq   $0x0,0xa0(%rsp)
  152f5e:	74 0e                                           	je     152f6e <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x151e>
  152f60:	48 8b bc 24 a8 00 00 00                         	mov    0xa8(%rsp),%rdi
  152f68:	ff 15 5a 1e 12 00                               	call   *0x121e5a(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152f6e:	48 8d bc 24 28 02 00 00                         	lea    0x228(%rsp),%rdi
  152f76:	e8 e5 e5 f3 ff                                  	call   91560 <core::ptr::drop_glue::<alloc::vec::Vec<alloc::vec::Vec<emuella_j2k_accel::OutputRect>>>>
  152f7b:	48 8d bc 24 f8 00 00 00                         	lea    0xf8(%rsp),%rdi
  152f83:	e8 a8 ec f3 ff                                  	call   91c30 <core::ptr::drop_glue::<alloc::vec::Vec<emuella_j2k_codestream::NativeDecompSubband>>>
  152f88:	48 8d bc 24 20 03 00 00                         	lea    0x320(%rsp),%rdi
  152f90:	e8 9b 7b f4 ff                                  	call   9ab30 <core::ptr::drop_glue::<emuella_j2k_codestream::scalable_lossless::parallel::BlockWorkers>>
  152f95:	48 8d bc 24 10 01 00 00                         	lea    0x110(%rsp),%rdi
  152f9d:	e8 3e 61 f4 ff                                  	call   990e0 <core::ptr::drop_glue::<emuella_j2k_tier1::CodeBlockEncodeScratch>>
  152fa2:	48 83 7c 24 48 00                               	cmpq   $0x0,0x48(%rsp)
  152fa8:	74 0b                                           	je     152fb5 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1565>
  152faa:	48 8b 7c 24 50                                  	mov    0x50(%rsp),%rdi
  152faf:	ff 15 13 1e 12 00                               	call   *0x121e13(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152fb5:	48 83 7c 24 18 00                               	cmpq   $0x0,0x18(%rsp)
  152fbb:	74 0b                                           	je     152fc8 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1578>
  152fbd:	48 8b 7c 24 30                                  	mov    0x30(%rsp),%rdi
  152fc2:	ff 15 00 1e 12 00                               	call   *0x121e00(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152fc8:	48 83 bc 24 d8 01 00 00 00                      	cmpq   $0x0,0x1d8(%rsp)
  152fd1:	74 0e                                           	je     152fe1 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x1591>
  152fd3:	48 8b bc 24 e0 01 00 00                         	mov    0x1e0(%rsp),%rdi
  152fdb:	ff 15 e7 1d 12 00                               	call   *0x121de7(%rip)        # 274dc8 <free@GLIBC_2.2.5>
  152fe1:	48 83 7c 24 40 00                               	cmpq   $0x0,0x40(%rsp)
  152fe7:	0f 84 9c f0 ff ff                               	je     152089 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x639>
  152fed:	48 8b 7c 24 08                                  	mov    0x8(%rsp),%rdi
  152ff2:	e9 8c f0 ff ff                                  	jmp    152083 <emuella_j2k_codestream::scalable_lossless::encode_lossless_d2+0x633>
