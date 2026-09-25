Disassembly of section .text:

00000000000865e0 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>>:
   865e0:	55                                              	push   %rbp
   865e1:	41 57                                           	push   %r15
   865e3:	41 56                                           	push   %r14
   865e5:	41 55                                           	push   %r13
   865e7:	41 54                                           	push   %r12
   865e9:	53                                              	push   %rbx
   865ea:	48 81 ec 78 01 00 00                            	sub    $0x178,%rsp
   865f1:	4d 89 c6                                        	mov    %r8,%r14
   865f4:	4d 8b 00                                        	mov    (%r8),%r8
   865f7:	45 8b 56 18                                     	mov    0x18(%r14),%r10d
   865fb:	4c 89 c3                                        	mov    %r8,%rbx
   865fe:	4c 01 d3                                        	add    %r10,%rbx
   86601:	0f 82 96 00 00 00                               	jb     8669d <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xbd>
   86607:	89 f5                                           	mov    %esi,%ebp
   86609:	4d 8b 5e 08                                     	mov    0x8(%r14),%r11
   8660d:	41 8b 76 1c                                     	mov    0x1c(%r14),%esi
   86611:	4d 89 dc                                        	mov    %r11,%r12
   86614:	49 01 f4                                        	add    %rsi,%r12
   86617:	0f 82 80 00 00 00                               	jb     8669d <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xbd>
   8661d:	4d 89 c5                                        	mov    %r8,%r13
   86620:	49 c1 ed 06                                     	shr    $0x6,%r13
   86624:	49 89 df                                        	mov    %rbx,%r15
   86627:	49 c1 ef 06                                     	shr    $0x6,%r15
   8662b:	89 d8                                           	mov    %ebx,%eax
   8662d:	83 e0 3f                                        	and    $0x3f,%eax
   86630:	48 83 f8 01                                     	cmp    $0x1,%rax
   86634:	49 83 df ff                                     	sbb    $0xffffffffffffffff,%r15
   86638:	4c 89 f8                                        	mov    %r15,%rax
   8663b:	4c 29 e8                                        	sub    %r13,%rax
   8663e:	72 5d                                           	jb     8669d <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xbd>
   86640:	48 3d ff ff 00 00                               	cmp    $0xffff,%rax
   86646:	77 55                                           	ja     8669d <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xbd>
   86648:	48 89 94 24 68 01 00 00                         	mov    %rdx,0x168(%rsp)
   86650:	48 89 8c 24 70 01 00 00                         	mov    %rcx,0x170(%rsp)
   86658:	48 89 44 24 30                                  	mov    %rax,0x30(%rsp)
   8665d:	44 89 4c 24 24                                  	mov    %r9d,0x24(%rsp)
   86662:	4c 89 44 24 18                                  	mov    %r8,0x18(%rsp)
   86667:	4c 89 9c 24 b8 00 00 00                         	mov    %r11,0xb8(%rsp)
   8666f:	49 c1 eb 06                                     	shr    $0x6,%r11
   86673:	4c 89 e0                                        	mov    %r12,%rax
   86676:	49 c1 ec 06                                     	shr    $0x6,%r12
   8667a:	48 89 84 24 60 01 00 00                         	mov    %rax,0x160(%rsp)
   86682:	83 e0 3f                                        	and    $0x3f,%eax
   86685:	48 83 f8 01                                     	cmp    $0x1,%rax
   86689:	49 83 dc ff                                     	sbb    $0xffffffffffffffff,%r12
   8668d:	4c 89 e0                                        	mov    %r12,%rax
   86690:	4c 29 d8                                        	sub    %r11,%rax
   86693:	72 08                                           	jb     8669d <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xbd>
   86695:	48 3d ff ff 00 00                               	cmp    $0xffff,%rax
   8669b:	76 27                                           	jbe    866c4 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xe4>
   8669d:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   866a7:	48 89 47 08                                     	mov    %rax,0x8(%rdi)
   866ab:	48 c7 07 01 00 00 00                            	movq   $0x1,(%rdi)
   866b2:	48 81 c4 78 01 00 00                            	add    $0x178,%rsp
   866b9:	5b                                              	pop    %rbx
   866ba:	41 5c                                           	pop    %r12
   866bc:	41 5d                                           	pop    %r13
   866be:	41 5e                                           	pop    %r14
   866c0:	41 5f                                           	pop    %r15
   866c2:	5d                                              	pop    %rbp
   866c3:	c3                                              	ret
   866c4:	48 89 c2                                        	mov    %rax,%rdx
   866c7:	48 0f af 54 24 30                               	imul   0x30(%rsp),%rdx
   866cd:	48 85 d2                                        	test   %rdx,%rdx
   866d0:	48 89 7c 24 10                                  	mov    %rdi,0x10(%rsp)
   866d5:	4c 89 9c 24 a8 00 00 00                         	mov    %r11,0xa8(%rsp)
   866dd:	4c 89 94 24 40 01 00 00                         	mov    %r10,0x140(%rsp)
   866e5:	48 89 b4 24 38 01 00 00                         	mov    %rsi,0x138(%rsp)
   866ed:	48 89 84 24 90 00 00 00                         	mov    %rax,0x90(%rsp)
   866f5:	74 32                                           	je     86729 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x149>
   866f7:	48 89 54 24 50                                  	mov    %rdx,0x50(%rsp)
   866fc:	48 89 d7                                        	mov    %rdx,%rdi
   866ff:	48 c1 e7 05                                     	shl    $0x5,%rdi
   86703:	48 89 7c 24 48                                  	mov    %rdi,0x48(%rsp)
   86708:	ff 15 aa e6 1e 00                               	call   *0x1ee6aa(%rip)        # 274db8 <malloc@GLIBC_2.2.5>
   8670e:	48 85 c0                                        	test   %rax,%rax
   86711:	0f 84 a6 08 00 00                               	je     86fbd <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x9dd>
   86717:	48 89 c1                                        	mov    %rax,%rcx
   8671a:	4c 8b 9c 24 a8 00 00 00                         	mov    0xa8(%rsp),%r11
   86722:	48 8b 54 24 50                                  	mov    0x50(%rsp),%rdx
   86727:	eb 05                                           	jmp    8672e <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x14e>
   86729:	b9 08 00 00 00                                  	mov    $0x8,%ecx
   8672e:	48 89 54 24 70                                  	mov    %rdx,0x70(%rsp)
   86733:	48 89 4c 24 78                                  	mov    %rcx,0x78(%rsp)
   86738:	48 c7 84 24 80 00 00 00 00 00 00 00             	movq   $0x0,0x80(%rsp)
   86744:	41 8b 46 10                                     	mov    0x10(%r14),%eax
   86748:	48 89 44 24 48                                  	mov    %rax,0x48(%rsp)
   8674d:	41 8b 46 14                                     	mov    0x14(%r14),%eax
   86751:	48 89 84 24 a0 00 00 00                         	mov    %rax,0xa0(%rsp)
   86759:	48 c7 44 24 58 00 00 00 00                      	movq   $0x0,0x58(%rsp)
   86762:	48 c7 44 24 60 01 00 00 00                      	movq   $0x1,0x60(%rsp)
   8676b:	48 c7 44 24 68 00 00 00 00                      	movq   $0x0,0x68(%rsp)
   86774:	4d 39 dc                                        	cmp    %r11,%r12
   86777:	75 0d                                           	jne    86786 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x1a6>
   86779:	45 0f b6 4e 22                                  	movzbl 0x22(%r14),%r9d
   8677e:	b0 01                                           	mov    $0x1,%al
   86780:	8b 7c 24 24                                     	mov    0x24(%rsp),%edi
   86784:	eb 33                                           	jmp    867b9 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x1d9>
   86786:	48 83 bc 24 c0 01 00 00 00                      	cmpq   $0x0,0x1c0(%rsp)
   8678f:	48 8d 44 24 58                                  	lea    0x58(%rsp),%rax
   86794:	48 0f 44 84 24 b0 01 00 00                      	cmove  0x1b0(%rsp),%rax
   8679d:	48 89 84 24 30 01 00 00                         	mov    %rax,0x130(%rsp)
   867a5:	45 0f b6 4e 22                                  	movzbl 0x22(%r14),%r9d
   867aa:	b0 01                                           	mov    $0x1,%al
   867ac:	4d 39 ef                                        	cmp    %r13,%r15
   867af:	8b 7c 24 24                                     	mov    0x24(%rsp),%edi
   867b3:	0f 85 83 00 00 00                               	jne    8683c <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x25c>
   867b9:	4c 8b 44 24 30                                  	mov    0x30(%rsp),%r8
   867be:	41 0f b7 4e 20                                  	movzwl 0x20(%r14),%ecx
   867c3:	0f 10 44 24 70                                  	movups 0x70(%rsp),%xmm0
   867c8:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
   867cd:	0f 11 46 08                                     	movups %xmm0,0x8(%rsi)
   867d1:	48 8b 94 24 80 00 00 00                         	mov    0x80(%rsp),%rdx
   867d9:	48 89 56 18                                     	mov    %rdx,0x18(%rsi)
   867dd:	48 8b 54 24 48                                  	mov    0x48(%rsp),%rdx
   867e2:	89 56 20                                        	mov    %edx,0x20(%rsi)
   867e5:	48 8b 94 24 a0 00 00 00                         	mov    0xa0(%rsp),%rdx
   867ed:	89 56 24                                        	mov    %edx,0x24(%rsi)
   867f0:	48 8b 94 24 40 01 00 00                         	mov    0x140(%rsp),%rdx
   867f8:	89 56 28                                        	mov    %edx,0x28(%rsi)
   867fb:	48 8b 94 24 38 01 00 00                         	mov    0x138(%rsp),%rdx
   86803:	89 56 2c                                        	mov    %edx,0x2c(%rsi)
   86806:	66 44 89 46 30                                  	mov    %r8w,0x30(%rsi)
   8680b:	48 8b 94 24 90 00 00 00                         	mov    0x90(%rsp),%rdx
   86813:	66 89 56 32                                     	mov    %dx,0x32(%rsi)
   86817:	44 88 4e 34                                     	mov    %r9b,0x34(%rsi)
   8681b:	66 89 4e 35                                     	mov    %cx,0x35(%rsi)
   8681f:	40 88 7e 37                                     	mov    %dil,0x37(%rsi)
   86823:	48 c7 06 00 00 00 00                            	movq   $0x0,(%rsi)
   8682a:	84 c0                                           	test   %al,%al
   8682c:	0f 85 80 fe ff ff                               	jne    866b2 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xd2>
   86832:	48 8b 7c 24 60                                  	mov    0x60(%rsp),%rdi
   86837:	e9 6b 07 00 00                                  	jmp    86fa7 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x9c7>
   8683c:	48 89 8c 24 98 00 00 00                         	mov    %rcx,0x98(%rsp)
   86844:	41 89 e8                                        	mov    %ebp,%r8d
   86847:	40 0f b6 c7                                     	movzbl %dil,%eax
   8684b:	48 c1 e0 20                                     	shl    $0x20,%rax
   8684f:	48 89 84 24 e8 00 00 00                         	mov    %rax,0xe8(%rsp)
   86857:	44 88 4c 24 0f                                  	mov    %r9b,0xf(%rsp)
   8685c:	41 0f b6 c1                                     	movzbl %r9b,%eax
   86860:	48 89 84 24 08 01 00 00                         	mov    %rax,0x108(%rsp)
   86868:	49 ba 00 00 00 00 00 00 00 04                   	movabs $0x400000000000000,%r10
   86872:	4c 89 d0                                        	mov    %r10,%rax
   86875:	4c 29 e8                                        	sub    %r13,%rax
   86878:	48 89 84 24 28 01 00 00                         	mov    %rax,0x128(%rsp)
   86880:	48 8b 44 24 18                                  	mov    0x18(%rsp),%rax
   86885:	48 f7 d0                                        	not    %rax
   86888:	48 c1 e8 06                                     	shr    $0x6,%rax
   8688c:	48 89 84 24 20 01 00 00                         	mov    %rax,0x120(%rsp)
   86894:	4d 29 da                                        	sub    %r11,%r10
   86897:	48 8b 84 24 b8 00 00 00                         	mov    0xb8(%rsp),%rax
   8689f:	48 89 c1                                        	mov    %rax,%rcx
   868a2:	48 f7 d1                                        	not    %rcx
   868a5:	48 c1 e9 06                                     	shr    $0x6,%rcx
   868a9:	48 89 8c 24 f0 00 00 00                         	mov    %rcx,0xf0(%rsp)
   868b1:	49 c1 e5 06                                     	shl    $0x6,%r13
   868b5:	45 31 ff                                        	xor    %r15d,%r15d
   868b8:	48 c7 44 24 40 00 00 00 00                      	movq   $0x0,0x40(%rsp)
   868c1:	4c 89 84 24 88 00 00 00                         	mov    %r8,0x88(%rsp)
   868c9:	4c 89 94 24 d8 00 00 00                         	mov    %r10,0xd8(%rsp)
   868d1:	4f 8d 0c 3b                                     	lea    (%r11,%r15,1),%r9
   868d5:	49 c1 e1 06                                     	shl    $0x6,%r9
   868d9:	4c 39 c8                                        	cmp    %r9,%rax
   868dc:	4c 89 ce                                        	mov    %r9,%rsi
   868df:	48 0f 47 f0                                     	cmova  %rax,%rsi
   868e3:	49 83 c1 40                                     	add    $0x40,%r9
   868e7:	48 8b 8c 24 60 01 00 00                         	mov    0x160(%rsp),%rcx
   868ef:	4c 39 c9                                        	cmp    %r9,%rcx
   868f2:	4c 0f 42 c9                                     	cmovb  %rcx,%r9
   868f6:	48 89 f1                                        	mov    %rsi,%rcx
   868f9:	48 29 c1                                        	sub    %rax,%rcx
   868fc:	48 8b bc 24 a0 00 00 00                         	mov    0xa0(%rsp),%rdi
   86904:	48 01 f9                                        	add    %rdi,%rcx
   86907:	48 89 c8                                        	mov    %rcx,%rax
   8690a:	49 f7 e0                                        	mul    %r8
   8690d:	48 89 84 24 18 01 00 00                         	mov    %rax,0x118(%rsp)
   86915:	0f 90 c0                                        	seto   %al
   86918:	4d 39 d7                                        	cmp    %r10,%r15
   8691b:	0f 84 32 05 00 00                               	je     86e53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   86921:	4c 3b bc 24 f0 00 00 00                         	cmp    0xf0(%rsp),%r15
   86929:	0f 84 83 06 00 00                               	je     86fb2 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x9d2>
   8692f:	49 39 f1                                        	cmp    %rsi,%r9
   86932:	0f 82 eb 04 00 00                               	jb     86e23 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x843>
   86938:	49 29 f1                                        	sub    %rsi,%r9
   8693b:	49 81 f9 ff ff 00 00                            	cmp    $0xffff,%r9
   86942:	0f 87 db 04 00 00                               	ja     86e23 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x843>
   86948:	48 39 f9                                        	cmp    %rdi,%rcx
   8694b:	4c 89 4c 24 38                                  	mov    %r9,0x38(%rsp)
   86950:	0f 82 8a 05 00 00                               	jb     86ee0 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x900>
   86956:	84 c0                                           	test   %al,%al
   86958:	0f 85 82 05 00 00                               	jne    86ee0 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x900>
   8695e:	4c 89 bc 24 00 01 00 00                         	mov    %r15,0x100(%rsp)
   86966:	49 8d 47 01                                     	lea    0x1(%r15),%rax
   8696a:	48 89 84 24 e0 00 00 00                         	mov    %rax,0xe0(%rsp)
   86972:	49 c1 e1 10                                     	shl    $0x10,%r9
   86976:	4c 03 8c 24 e8 00 00 00                         	add    0xe8(%rsp),%r9
   8697e:	4c 89 8c 24 10 01 00 00                         	mov    %r9,0x110(%rsp)
   86986:	4c 8b 64 24 40                                  	mov    0x40(%rsp),%r12
   8698b:	49 c1 e4 05                                     	shl    $0x5,%r12
   8698f:	4c 89 ee                                        	mov    %r13,%rsi
   86992:	31 d2                                           	xor    %edx,%edx
   86994:	e9 9e 00 00 00                                  	jmp    86a37 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x457>
   86999:	4c 8b 44 24 50                                  	mov    0x50(%rsp),%r8
   8699e:	49 8d 40 01                                     	lea    0x1(%r8),%rax
   869a2:	44 89 e1                                        	mov    %r12d,%ecx
   869a5:	41 c1 ec 18                                     	shr    $0x18,%r12d
   869a9:	41 80 e4 01                                     	and    $0x1,%r12b
   869ad:	48 8b 94 24 98 00 00 00                         	mov    0x98(%rsp),%rdx
   869b5:	48 8b b4 24 50 01 00 00                         	mov    0x150(%rsp),%rsi
   869bd:	48 89 34 3a                                     	mov    %rsi,(%rdx,%rdi,1)
   869c1:	48 8b b4 24 48 01 00 00                         	mov    0x148(%rsp),%rsi
   869c9:	48 89 74 3a 08                                  	mov    %rsi,0x8(%rdx,%rdi,1)
   869ce:	66 44 89 44 3a 10                               	mov    %r8w,0x10(%rdx,%rdi,1)
   869d4:	48 8b b4 24 00 01 00 00                         	mov    0x100(%rsp),%rsi
   869dc:	66 89 74 3a 12                                  	mov    %si,0x12(%rdx,%rdi,1)
   869e1:	66 89 6c 3a 14                                  	mov    %bp,0x14(%rdx,%rdi,1)
   869e6:	48 8b 74 24 38                                  	mov    0x38(%rsp),%rsi
   869eb:	66 89 74 3a 16                                  	mov    %si,0x16(%rdx,%rdi,1)
   869f0:	66 89 4c 3a 18                                  	mov    %cx,0x18(%rdx,%rdi,1)
   869f5:	44 88 7c 3a 1a                                  	mov    %r15b,0x1a(%rdx,%rdi,1)
   869fa:	44 88 64 3a 1b                                  	mov    %r12b,0x1b(%rdx,%rdi,1)
   869ff:	48 8b 4c 24 40                                  	mov    0x40(%rsp),%rcx
   86a04:	4c 01 c1                                        	add    %r8,%rcx
   86a07:	48 ff c1                                        	inc    %rcx
   86a0a:	48 89 8c 24 80 00 00 00                         	mov    %rcx,0x80(%rsp)
   86a12:	48 83 c7 20                                     	add    $0x20,%rdi
   86a16:	48 89 c2                                        	mov    %rax,%rdx
   86a19:	48 39 44 24 30                                  	cmp    %rax,0x30(%rsp)
   86a1e:	4c 8b 9c 24 a8 00 00 00                         	mov    0xa8(%rsp),%r11
   86a26:	49 89 fc                                        	mov    %rdi,%r12
   86a29:	48 8b b4 24 58 01 00 00                         	mov    0x158(%rsp),%rsi
   86a31:	0f 84 23 03 00 00                               	je     86d5a <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x77a>
   86a37:	48 39 94 24 28 01 00 00                         	cmp    %rdx,0x128(%rsp)
   86a3f:	0f 84 0e 04 00 00                               	je     86e53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   86a45:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
   86a4a:	48 39 f1                                        	cmp    %rsi,%rcx
   86a4d:	48 89 f0                                        	mov    %rsi,%rax
   86a50:	48 0f 47 c1                                     	cmova  %rcx,%rax
   86a54:	48 89 54 24 50                                  	mov    %rdx,0x50(%rsp)
   86a59:	48 39 94 24 20 01 00 00                         	cmp    %rdx,0x120(%rsp)
   86a61:	4c 8b 8c 24 88 00 00 00                         	mov    0x88(%rsp),%r9
   86a69:	0f 84 e4 03 00 00                               	je     86e53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   86a6f:	48 83 c6 40                                     	add    $0x40,%rsi
   86a73:	48 39 f3                                        	cmp    %rsi,%rbx
   86a76:	48 89 f5                                        	mov    %rsi,%rbp
   86a79:	48 0f 42 eb                                     	cmovb  %rbx,%rbp
   86a7d:	48 29 c5                                        	sub    %rax,%rbp
   86a80:	0f 82 cd 03 00 00                               	jb     86e53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   86a86:	48 81 fd ff ff 00 00                            	cmp    $0xffff,%rbp
   86a8d:	0f 87 c0 03 00 00                               	ja     86e53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   86a93:	48 89 e9                                        	mov    %rbp,%rcx
   86a96:	48 0f af 4c 24 38                               	imul   0x38(%rsp),%rcx
   86a9c:	48 81 c1 ff ef ff ff                            	add    $0xffffffffffffefff,%rcx
   86aa3:	48 81 f9 00 f0 ff ff                            	cmp    $0xfffffffffffff000,%rcx
   86aaa:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   86aaf:	0f 82 81 04 00 00                               	jb     86f36 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x956>
   86ab5:	48 2b 44 24 18                                  	sub    0x18(%rsp),%rax
   86aba:	48 03 44 24 48                                  	add    0x48(%rsp),%rax
   86abf:	0f 82 a9 04 00 00                               	jb     86f6e <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x98e>
   86ac5:	48 03 84 24 18 01 00 00                         	add    0x118(%rsp),%rax
   86acd:	0f 82 9b 04 00 00                               	jb     86f6e <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x98e>
   86ad3:	4c 89 a4 24 b0 00 00 00                         	mov    %r12,0xb0(%rsp)
   86adb:	48 8b 94 24 70 01 00 00                         	mov    0x170(%rsp),%rdx
   86ae3:	48 29 c2                                        	sub    %rax,%rdx
   86ae6:	0f 82 82 04 00 00                               	jb     86f6e <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x98e>
   86aec:	48 89 b4 24 58 01 00 00                         	mov    %rsi,0x158(%rsp)
   86af4:	48 8b 8c 24 68 01 00 00                         	mov    0x168(%rsp),%rcx
   86afc:	48 8d 34 81                                     	lea    (%rcx,%rax,4),%rsi
   86b00:	48 8b 84 24 b0 01 00 00                         	mov    0x1b0(%rsp),%rax
   86b08:	48 8b 40 10                                     	mov    0x10(%rax),%rax
   86b0c:	48 89 84 24 50 01 00 00                         	mov    %rax,0x150(%rsp)
   86b14:	48 c7 44 24 68 00 00 00 00                      	movq   $0x0,0x68(%rsp)
   86b1d:	48 8b 84 24 08 01 00 00                         	mov    0x108(%rsp),%rax
   86b25:	48 8d 0d 74 ae f8 ff                            	lea    -0x7518c(%rip),%rcx        # 119a0 <anon.02ffda8f51e6076a26652e0a4fd6fa8d.1589.llvm.5406940975955002095+0x20>
   86b2c:	4c 8b 04 c1                                     	mov    (%rcx,%rax,8),%r8
   86b30:	4c 0b 84 24 10 01 00 00                         	or     0x110(%rsp),%r8
   86b38:	49 09 e8                                        	or     %rbp,%r8
   86b3b:	48 8b 84 24 b8 01 00 00                         	mov    0x1b8(%rsp),%rax
   86b43:	48 89 04 24                                     	mov    %rax,(%rsp)
   86b47:	48 8d bc 24 c0 00 00 00                         	lea    0xc0(%rsp),%rdi
   86b4f:	4c 89 c9                                        	mov    %r9,%rcx
   86b52:	4c 8b 8c 24 30 01 00 00                         	mov    0x130(%rsp),%r9
   86b5a:	ff 15 28 e7 1e 00                               	call   *0x1ee728(%rip)        # 275288 <_DYNAMIC+0x6d8>
   86b60:	0f b6 84 24 c0 00 00 00                         	movzbl 0xc0(%rsp),%eax
   86b68:	4c 8b bc 24 d0 00 00 00                         	mov    0xd0(%rsp),%r15
   86b70:	3c ff                                           	cmp    $0xff,%al
   86b72:	0f 85 35 02 00 00                               	jne    86dad <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x7cd>
   86b78:	48 8b 84 24 c8 00 00 00                         	mov    0xc8(%rsp),%rax
   86b80:	48 89 84 24 48 01 00 00                         	mov    %rax,0x148(%rsp)
   86b88:	48 8b 84 24 c0 01 00 00                         	mov    0x1c0(%rsp),%rax
   86b90:	a8 01                                           	test   $0x1,%al
   86b92:	0f 84 d4 00 00 00                               	je     86c6c <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x68c>
   86b98:	48 8b 54 24 68                                  	mov    0x68(%rsp),%rdx
   86b9d:	48 8b 8c 24 b0 01 00 00                         	mov    0x1b0(%rsp),%rcx
   86ba5:	4c 8b 61 10                                     	mov    0x10(%rcx),%r12
   86ba9:	49 8d 04 14                                     	lea    (%r12,%rdx,1),%rax
   86bad:	48 3b 84 24 c8 01 00 00                         	cmp    0x1c8(%rsp),%rax
   86bb5:	0f 87 5a 02 00 00                               	ja     86e15 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x835>
   86bbb:	48 8b 31                                        	mov    (%rcx),%rsi
   86bbe:	48 39 f0                                        	cmp    %rsi,%rax
   86bc1:	76 50                                           	jbe    86c13 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x633>
   86bc3:	4c 8d 14 36                                     	lea    (%rsi,%rsi,1),%r10
   86bc7:	48 8b 8c 24 c8 01 00 00                         	mov    0x1c8(%rsp),%rcx
   86bcf:	4c 39 d1                                        	cmp    %r10,%rcx
   86bd2:	4c 0f 42 d1                                     	cmovb  %rcx,%r10
   86bd6:	49 39 c2                                        	cmp    %rax,%r10
   86bd9:	4c 0f 46 d0                                     	cmovbe %rax,%r10
   86bdd:	4c 89 d0                                        	mov    %r10,%rax
   86be0:	4c 29 e0                                        	sub    %r12,%rax
   86be3:	48 89 f1                                        	mov    %rsi,%rcx
   86be6:	4c 29 e1                                        	sub    %r12,%rcx
   86be9:	48 39 c8                                        	cmp    %rcx,%rax
   86bec:	0f 87 cf 00 00 00                               	ja     86cc1 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x6e1>
   86bf2:	48 8b bc 24 b0 01 00 00                         	mov    0x1b0(%rsp),%rdi
   86bfa:	48 3b b4 24 c8 01 00 00                         	cmp    0x1c8(%rsp),%rsi
   86c02:	0f 87 5f 02 00 00                               	ja     86e67 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x887>
   86c08:	48 8b 54 24 68                                  	mov    0x68(%rsp),%rdx
   86c0d:	4c 8b 67 10                                     	mov    0x10(%rdi),%r12
   86c11:	eb 08                                           	jmp    86c1b <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x63b>
   86c13:	48 8b bc 24 b0 01 00 00                         	mov    0x1b0(%rsp),%rdi
   86c1b:	48 8b 44 24 60                                  	mov    0x60(%rsp),%rax
   86c20:	48 89 84 24 f8 00 00 00                         	mov    %rax,0xf8(%rsp)
   86c28:	4c 29 e6                                        	sub    %r12,%rsi
   86c2b:	48 39 f2                                        	cmp    %rsi,%rdx
   86c2e:	0f 87 f8 00 00 00                               	ja     86d2c <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x74c>
   86c34:	48 85 d2                                        	test   %rdx,%rdx
   86c37:	74 2c                                           	je     86c65 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x685>
   86c39:	48 89 54 24 28                                  	mov    %rdx,0x28(%rsp)
   86c3e:	48 8b 7f 08                                     	mov    0x8(%rdi),%rdi
   86c42:	4c 01 e7                                        	add    %r12,%rdi
   86c45:	48 8b b4 24 f8 00 00 00                         	mov    0xf8(%rsp),%rsi
   86c4d:	48 8b 54 24 28                                  	mov    0x28(%rsp),%rdx
   86c52:	ff 15 80 e1 1e 00                               	call   *0x1ee180(%rip)        # 274dd8 <memcpy@GLIBC_2.14>
   86c58:	48 8b 54 24 28                                  	mov    0x28(%rsp),%rdx
   86c5d:	48 8b bc 24 b0 01 00 00                         	mov    0x1b0(%rsp),%rdi
   86c65:	49 01 d4                                        	add    %rdx,%r12
   86c68:	4c 89 67 10                                     	mov    %r12,0x10(%rdi)
   86c6c:	4d 89 fc                                        	mov    %r15,%r12
   86c6f:	41 c1 ef 10                                     	shr    $0x10,%r15d
   86c73:	41 fe c7                                        	inc    %r15b
   86c76:	0f 84 2f 02 00 00                               	je     86eab <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x8cb>
   86c7c:	48 8b 44 24 40                                  	mov    0x40(%rsp),%rax
   86c81:	48 8b 4c 24 50                                  	mov    0x50(%rsp),%rcx
   86c86:	48 01 c8                                        	add    %rcx,%rax
   86c89:	48 3b 44 24 70                                  	cmp    0x70(%rsp),%rax
   86c8e:	48 8b bc 24 b0 00 00 00                         	mov    0xb0(%rsp),%rdi
   86c96:	0f 85 fd fc ff ff                               	jne    86999 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x3b9>
   86c9c:	48 8d 7c 24 70                                  	lea    0x70(%rsp),%rdi
   86ca1:	ff 15 e9 e5 1e 00                               	call   *0x1ee5e9(%rip)        # 275290 <_DYNAMIC+0x6e0>
   86ca7:	48 8b 44 24 78                                  	mov    0x78(%rsp),%rax
   86cac:	48 89 84 24 98 00 00 00                         	mov    %rax,0x98(%rsp)
   86cb4:	48 8b bc 24 b0 00 00 00                         	mov    0xb0(%rsp),%rdi
   86cbc:	e9 d8 fc ff ff                                  	jmp    86999 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x3b9>
   86cc1:	4c 89 7c 24 28                                  	mov    %r15,0x28(%rsp)
   86cc6:	4c 8b bc 24 b0 01 00 00                         	mov    0x1b0(%rsp),%r15
   86cce:	49 8b 57 08                                     	mov    0x8(%r15),%rdx
   86cd2:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   86cd8:	41 b9 01 00 00 00                               	mov    $0x1,%r9d
   86cde:	48 8d bc 24 c0 00 00 00                         	lea    0xc0(%rsp),%rdi
   86ce6:	4c 89 d1                                        	mov    %r10,%rcx
   86ce9:	4d 89 d4                                        	mov    %r10,%r12
   86cec:	e8 1f 3f 0c 00                                  	call   14ac10 <<alloc::raw_vec::RawVecInner>::finish_grow>
   86cf1:	80 bc 24 c0 00 00 00 00                         	cmpb   $0x0,0xc0(%rsp)
   86cf9:	0f 85 d5 01 00 00                               	jne    86ed4 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x8f4>
   86cff:	48 8b 84 24 c8 00 00 00                         	mov    0xc8(%rsp),%rax
   86d07:	49 89 47 08                                     	mov    %rax,0x8(%r15)
   86d0b:	4d 89 27                                        	mov    %r12,(%r15)
   86d0e:	4c 89 ff                                        	mov    %r15,%rdi
   86d11:	4c 8b 7c 24 28                                  	mov    0x28(%rsp),%r15
   86d16:	4c 89 e6                                        	mov    %r12,%rsi
   86d19:	48 3b b4 24 c8 01 00 00                         	cmp    0x1c8(%rsp),%rsi
   86d21:	0f 86 e1 fe ff ff                               	jbe    86c08 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x628>
   86d27:	e9 3b 01 00 00                                  	jmp    86e67 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x887>
   86d2c:	b9 01 00 00 00                                  	mov    $0x1,%ecx
   86d31:	41 b8 01 00 00 00                               	mov    $0x1,%r8d
   86d37:	4c 89 e6                                        	mov    %r12,%rsi
   86d3a:	48 89 54 24 28                                  	mov    %rdx,0x28(%rsp)
   86d3f:	e8 ac 49 02 00                                  	call   ab6f0 <<alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>>
   86d44:	48 8b bc 24 b0 01 00 00                         	mov    0x1b0(%rsp),%rdi
   86d4c:	4c 8b 67 10                                     	mov    0x10(%rdi),%r12
   86d50:	48 8b 54 24 28                                  	mov    0x28(%rsp),%rdx
   86d55:	e9 df fe ff ff                                  	jmp    86c39 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x659>
   86d5a:	48 01 44 24 40                                  	add    %rax,0x40(%rsp)
   86d5f:	48 8b 84 24 e0 00 00 00                         	mov    0xe0(%rsp),%rax
   86d67:	49 89 c7                                        	mov    %rax,%r15
   86d6a:	48 3b 84 24 90 00 00 00                         	cmp    0x90(%rsp),%rax
   86d72:	48 8b 84 24 b8 00 00 00                         	mov    0xb8(%rsp),%rax
   86d7a:	4c 8b 84 24 88 00 00 00                         	mov    0x88(%rsp),%r8
   86d82:	4c 8b 94 24 d8 00 00 00                         	mov    0xd8(%rsp),%r10
   86d8a:	0f 85 41 fb ff ff                               	jne    868d1 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x2f1>
   86d90:	48 83 7c 24 58 00                               	cmpq   $0x0,0x58(%rsp)
   86d96:	0f 94 c0                                        	sete   %al
   86d99:	8b 7c 24 24                                     	mov    0x24(%rsp),%edi
   86d9d:	4c 8b 44 24 30                                  	mov    0x30(%rsp),%r8
   86da2:	44 0f b6 4c 24 0f                               	movzbl 0xf(%rsp),%r9d
   86da8:	e9 11 fa ff ff                                  	jmp    867be <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x1de>
   86dad:	48 b9 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rcx
   86db7:	48 8d 51 fe                                     	lea    -0x2(%rcx),%rdx
   86dbb:	48 ff c9                                        	dec    %rcx
   86dbe:	3c 03                                           	cmp    $0x3,%al
   86dc0:	48 0f 44 ca                                     	cmove  %rdx,%rcx
   86dc4:	48 8d 05 b3 cb f8 ff                            	lea    -0x7344d(%rip),%rax        # 1397e <crossbeam_epoch::guard::unprotected::UNPROTECTED+0xc1e>
   86dcb:	48 0f 44 84 24 c8 00 00 00                      	cmove  0xc8(%rsp),%rax
   86dd4:	ba 43 00 00 00                                  	mov    $0x43,%edx
   86dd9:	49 0f 44 d7                                     	cmove  %r15,%rdx
   86ddd:	48 8b 74 24 10                                  	mov    0x10(%rsp),%rsi
   86de2:	48 89 4e 08                                     	mov    %rcx,0x8(%rsi)
   86de6:	48 89 f1                                        	mov    %rsi,%rcx
   86de9:	48 c7 46 10 00 00 00 00                         	movq   $0x0,0x10(%rsi)
   86df1:	66 c7 46 1a 00 00                               	movw   $0x0,0x1a(%rsi)
   86df7:	c7 46 1c 00 00 00 00                            	movl   $0x0,0x1c(%rsi)
   86dfe:	48 89 46 20                                     	mov    %rax,0x20(%rsi)
   86e02:	48 89 56 28                                     	mov    %rdx,0x28(%rsi)
   86e06:	66 c7 46 30 ff ff                               	movw   $0xffff,0x30(%rsi)
   86e0c:	c6 46 34 0b                                     	movb   $0xb,0x34(%rsi)
   86e10:	e9 67 01 00 00                                  	jmp    86f7c <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x99c>
   86e15:	b8 32 00 00 00                                  	mov    $0x32,%eax
   86e1a:	48 8d 35 fb 72 f9 ff                            	lea    -0x68d05(%rip),%rsi        # 1e11c <anon.38d8fac72642096468ff551e1950d360.1255.llvm.10326835611459877753+0x45c>
   86e21:	eb 50                                           	jmp    86e73 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x893>
   86e23:	48 83 7c 24 18 bf                               	cmpq   $0xffffffffffffffbf,0x18(%rsp)
   86e29:	77 28                                           	ja     86e53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   86e2b:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
   86e30:	48 89 c8                                        	mov    %rcx,%rax
   86e33:	48 83 e0 c0                                     	and    $0xffffffffffffffc0,%rax
   86e37:	48 83 c0 40                                     	add    $0x40,%rax
   86e3b:	48 39 c3                                        	cmp    %rax,%rbx
   86e3e:	48 0f 42 c3                                     	cmovb  %rbx,%rax
   86e42:	48 29 c8                                        	sub    %rcx,%rax
   86e45:	0f 92 c1                                        	setb   %cl
   86e48:	48 3d 00 00 01 00                               	cmp    $0x10000,%rax
   86e4e:	0f 93 c0                                        	setae  %al
   86e51:	08 c8                                           	or     %cl,%al
   86e53:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   86e5d:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   86e62:	e9 11 01 00 00                                  	jmp    86f78 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x998>
   86e67:	b8 35 00 00 00                                  	mov    $0x35,%eax
   86e6c:	48 8d 35 74 72 f9 ff                            	lea    -0x68d8c(%rip),%rsi        # 1e0e7 <anon.38d8fac72642096468ff551e1950d360.1255.llvm.10326835611459877753+0x427>
   86e73:	48 b9 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rcx
   86e7d:	48 ff c9                                        	dec    %rcx
   86e80:	48 8b 54 24 10                                  	mov    0x10(%rsp),%rdx
   86e85:	48 89 4a 08                                     	mov    %rcx,0x8(%rdx)
   86e89:	48 89 d1                                        	mov    %rdx,%rcx
   86e8c:	48 c7 42 10 00 00 00 00                         	movq   $0x0,0x10(%rdx)
   86e94:	48 89 72 20                                     	mov    %rsi,0x20(%rdx)
   86e98:	48 89 42 28                                     	mov    %rax,0x28(%rdx)
   86e9c:	66 c7 42 30 04 00                               	movw   $0x4,0x30(%rdx)
   86ea2:	c6 42 34 0a                                     	movb   $0xa,0x34(%rdx)
   86ea6:	e9 d1 00 00 00                                  	jmp    86f7c <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x99c>
   86eab:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   86eb5:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   86eba:	48 89 41 08                                     	mov    %rax,0x8(%rcx)
   86ebe:	c6 41 17 00                                     	movb   $0x0,0x17(%rcx)
   86ec2:	66 c7 41 15 00 00                               	movw   $0x0,0x15(%rcx)
   86ec8:	c7 41 11 00 00 00 00                            	movl   $0x0,0x11(%rcx)
   86ecf:	e9 a8 00 00 00                                  	jmp    86f7c <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x99c>
   86ed4:	48 b9 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rcx
   86ede:	eb a0                                           	jmp    86e80 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x8a0>
   86ee0:	48 83 7c 24 18 bf                               	cmpq   $0xffffffffffffffbf,0x18(%rsp)
   86ee6:	0f 87 67 ff ff ff                               	ja     86e53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   86eec:	48 8b 4c 24 18                                  	mov    0x18(%rsp),%rcx
   86ef1:	48 89 c8                                        	mov    %rcx,%rax
   86ef4:	48 83 e0 c0                                     	and    $0xffffffffffffffc0,%rax
   86ef8:	48 83 c0 40                                     	add    $0x40,%rax
   86efc:	48 39 c3                                        	cmp    %rax,%rbx
   86eff:	48 0f 42 c3                                     	cmovb  %rbx,%rax
   86f03:	48 29 c8                                        	sub    %rcx,%rax
   86f06:	0f 82 47 ff ff ff                               	jb     86e53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   86f0c:	48 3d ff ff 00 00                               	cmp    $0xffff,%rax
   86f12:	0f 87 3b ff ff ff                               	ja     86e53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   86f18:	48 8b 4c 24 38                                  	mov    0x38(%rsp),%rcx
   86f1d:	48 0f af c8                                     	imul   %rax,%rcx
   86f21:	48 81 c1 ff ef ff ff                            	add    $0xffffffffffffefff,%rcx
   86f28:	48 81 f9 00 f0 ff ff                            	cmp    $0xfffffffffffff000,%rcx
   86f2f:	48 8b 4c 24 10                                  	mov    0x10(%rsp),%rcx
   86f34:	73 38                                           	jae    86f6e <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x98e>
   86f36:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   86f40:	48 ff c8                                        	dec    %rax
   86f43:	48 89 41 08                                     	mov    %rax,0x8(%rcx)
   86f47:	48 c7 41 10 00 00 00 00                         	movq   $0x0,0x10(%rcx)
   86f4f:	48 8d 05 28 ca f8 ff                            	lea    -0x735d8(%rip),%rax        # 1397e <crossbeam_epoch::guard::unprotected::UNPROTECTED+0xc1e>
   86f56:	48 89 41 20                                     	mov    %rax,0x20(%rcx)
   86f5a:	48 c7 41 28 43 00 00 00                         	movq   $0x43,0x28(%rcx)
   86f62:	66 c7 41 30 ff ff                               	movw   $0xffff,0x30(%rcx)
   86f68:	c6 41 34 0b                                     	movb   $0xb,0x34(%rcx)
   86f6c:	eb 0e                                           	jmp    86f7c <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x99c>
   86f6e:	48 b8 03 00 00 00 00 00 00 80                   	movabs $0x8000000000000003,%rax
   86f78:	48 89 41 08                                     	mov    %rax,0x8(%rcx)
   86f7c:	48 c7 01 01 00 00 00                            	movq   $0x1,(%rcx)
   86f83:	48 83 7c 24 58 00                               	cmpq   $0x0,0x58(%rsp)
   86f89:	74 0b                                           	je     86f96 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x9b6>
   86f8b:	48 8b 7c 24 60                                  	mov    0x60(%rsp),%rdi
   86f90:	ff 15 32 de 1e 00                               	call   *0x1ede32(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   86f96:	48 83 7c 24 70 00                               	cmpq   $0x0,0x70(%rsp)
   86f9c:	0f 84 10 f7 ff ff                               	je     866b2 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xd2>
   86fa2:	48 8b 7c 24 78                                  	mov    0x78(%rsp),%rdi
   86fa7:	ff 15 1b de 1e 00                               	call   *0x1ede1b(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   86fad:	e9 00 f7 ff ff                                  	jmp    866b2 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xd2>
   86fb2:	48 83 7c 24 18 bf                               	cmpq   $0xffffffffffffffbf,0x18(%rsp)
   86fb8:	e9 96 fe ff ff                                  	jmp    86e53 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0x873>
   86fbd:	bf 08 00 00 00                                  	mov    $0x8,%edi
   86fc2:	48 8b 74 24 48                                  	mov    0x48(%rsp),%rsi
   86fc7:	ff 15 33 de 1e 00                               	call   *0x1ede33(%rip)        # 274e00 <_DYNAMIC+0x250>
   86fcd:	48 89 c3                                        	mov    %rax,%rbx
   86fd0:	48 83 7c 24 58 00                               	cmpq   $0x0,0x58(%rsp)
   86fd6:	75 10                                           	jne    86fe8 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xa08>
   86fd8:	48 83 7c 24 70 00                               	cmpq   $0x0,0x70(%rsp)
   86fde:	75 1b                                           	jne    86ffb <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xa1b>
   86fe0:	48 89 df                                        	mov    %rbx,%rdi
   86fe3:	e8 b8 51 1e 00                                  	call   26c1a0 <_Unwind_Resume@plt>
   86fe8:	48 8b 7c 24 60                                  	mov    0x60(%rsp),%rdi
   86fed:	ff 15 d5 dd 1e 00                               	call   *0x1eddd5(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   86ff3:	48 83 7c 24 70 00                               	cmpq   $0x0,0x70(%rsp)
   86ff9:	74 e5                                           	je     86fe0 <emuella_j2k_codestream::encode_decomp_subband_with_output_limit::<false>+0xa00>
   86ffb:	48 8b 7c 24 78                                  	mov    0x78(%rsp),%rdi
   87000:	ff 15 c2 dd 1e 00                               	call   *0x1eddc2(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   87006:	48 89 df                                        	mov    %rbx,%rdi
   87009:	e8 92 51 1e 00                                  	call   26c1a0 <_Unwind_Resume@plt>
