Disassembly of section .text:

000000000006e8a0 <classic_compare_worker::encode>:
   6e8a0:	55                                              	push   %rbp
   6e8a1:	41 57                                           	push   %r15
   6e8a3:	41 56                                           	push   %r14
   6e8a5:	41 55                                           	push   %r13
   6e8a7:	41 54                                           	push   %r12
   6e8a9:	53                                              	push   %rbx
   6e8aa:	48 81 ec c8 00 00 00                            	sub    $0xc8,%rsp
   6e8b1:	49 89 cd                                        	mov    %rcx,%r13
   6e8b4:	49 89 d4                                        	mov    %rdx,%r12
   6e8b7:	49 89 f7                                        	mov    %rsi,%r15
   6e8ba:	48 83 7e 10 07                                  	cmpq   $0x7,0x10(%rsi)
   6e8bf:	75 1b                                           	jne    6e8dc <classic_compare_worker::encode+0x3c>
   6e8c1:	49 8b 47 08                                     	mov    0x8(%r15),%rax
   6e8c5:	b9 65 6d 75 65                                  	mov    $0x65756d65,%ecx
   6e8ca:	33 08                                           	xor    (%rax),%ecx
   6e8cc:	ba 65 6c 6c 61                                  	mov    $0x616c6c65,%edx
   6e8d1:	33 50 03                                        	xor    0x3(%rax),%edx
   6e8d4:	09 ca                                           	or     %ecx,%edx
   6e8d6:	0f 84 d0 00 00 00                               	je     6e9ac <classic_compare_worker::encode+0x10c>
   6e8dc:	41 0f b6 af ca 00 00 00                         	movzbl 0xca(%r15),%ebp
   6e8e4:	83 fd 08                                        	cmp    $0x8,%ebp
   6e8e7:	75 36                                           	jne    6e91f <classic_compare_worker::encode+0x7f>
   6e8e9:	48 89 7c 24 38                                  	mov    %rdi,0x38(%rsp)
   6e8ee:	4a 8d 1c ad 00 00 00 00                         	lea    0x0(,%r13,4),%rbx
   6e8f6:	4c 89 e8                                        	mov    %r13,%rax
   6e8f9:	48 c1 e8 3e                                     	shr    $0x3e,%rax
   6e8fd:	0f 95 c0                                        	setne  %al
   6e900:	48 b9 fc ff ff ff ff ff ff 7f                   	movabs $0x7ffffffffffffffc,%rcx
   6e90a:	48 39 cb                                        	cmp    %rcx,%rbx
   6e90d:	0f 97 c1                                        	seta   %cl
   6e910:	08 c1                                           	or     %al,%cl
   6e912:	74 2a                                           	je     6e93e <classic_compare_worker::encode+0x9e>
   6e914:	31 ff                                           	xor    %edi,%edi
   6e916:	48 89 de                                        	mov    %rbx,%rsi
   6e919:	ff 15 e1 64 20 00                               	call   *0x2064e1(%rip)        # 274e00 <_DYNAMIC+0x250>
   6e91f:	4c 89 e9                                        	mov    %r13,%rcx
   6e922:	48 d1 e9                                        	shr    $1,%rcx
   6e925:	48 8d 34 8d 00 00 00 00                         	lea    0x0(,%rcx,4),%rsi
   6e92d:	4c 89 e8                                        	mov    %r13,%rax
   6e930:	48 c1 e8 3e                                     	shr    $0x3e,%rax
   6e934:	74 3c                                           	je     6e972 <classic_compare_worker::encode+0xd2>
   6e936:	31 ff                                           	xor    %edi,%edi
   6e938:	ff 15 c2 64 20 00                               	call   *0x2064c2(%rip)        # 274e00 <_DYNAMIC+0x250>
   6e93e:	48 85 db                                        	test   %rbx,%rbx
   6e941:	0f 84 a7 00 00 00                               	je     6e9ee <classic_compare_worker::encode+0x14e>
   6e947:	48 89 df                                        	mov    %rbx,%rdi
   6e94a:	ff 15 68 64 20 00                               	call   *0x206468(%rip)        # 274db8 <malloc@GLIBC_2.2.5>
   6e950:	49 89 c6                                        	mov    %rax,%r14
   6e953:	4c 89 ea                                        	mov    %r13,%rdx
   6e956:	48 85 c0                                        	test   %rax,%rax
   6e959:	0f 84 cb 06 00 00                               	je     6f02a <classic_compare_worker::encode+0x78a>
   6e95f:	4d 85 ed                                        	test   %r13,%r13
   6e962:	48 89 54 24 30                                  	mov    %rdx,0x30(%rsp)
   6e967:	0f 85 97 00 00 00                               	jne    6ea04 <classic_compare_worker::encode+0x164>
   6e96d:	e9 b2 04 00 00                                  	jmp    6ee24 <classic_compare_worker::encode+0x584>
   6e972:	48 89 7c 24 38                                  	mov    %rdi,0x38(%rsp)
   6e977:	48 89 4c 24 30                                  	mov    %rcx,0x30(%rsp)
   6e97c:	48 85 c9                                        	test   %rcx,%rcx
   6e97f:	0f 84 13 01 00 00                               	je     6ea98 <classic_compare_worker::encode+0x1f8>
   6e985:	48 89 f3                                        	mov    %rsi,%rbx
   6e988:	48 89 f7                                        	mov    %rsi,%rdi
   6e98b:	ff 15 27 64 20 00                               	call   *0x206427(%rip)        # 274db8 <malloc@GLIBC_2.2.5>
   6e991:	49 89 c6                                        	mov    %rax,%r14
   6e994:	48 85 c0                                        	test   %rax,%rax
   6e997:	0f 84 8d 06 00 00                               	je     6f02a <classic_compare_worker::encode+0x78a>
   6e99d:	49 83 fd 02                                     	cmp    $0x2,%r13
   6e9a1:	0f 83 01 01 00 00                               	jae    6eaa8 <classic_compare_worker::encode+0x208>
   6e9a7:	e9 78 04 00 00                                  	jmp    6ee24 <classic_compare_worker::encode+0x584>
   6e9ac:	48 89 fb                                        	mov    %rdi,%rbx
   6e9af:	49 89 e6                                        	mov    %rsp,%r14
   6e9b2:	4c 89 f7                                        	mov    %r14,%rdi
   6e9b5:	4c 89 fe                                        	mov    %r15,%rsi
   6e9b8:	e8 d3 16 00 00                                  	call   70090 <<classic_compare_worker::Request>::info>
   6e9bd:	48 8b 04 24                                     	mov    (%rsp),%rax
   6e9c1:	0f 10 44 24 08                                  	movups 0x8(%rsp),%xmm0
   6e9c6:	0f 29 44 24 40                                  	movaps %xmm0,0x40(%rsp)
   6e9cb:	48 83 f8 ff                                     	cmp    $0xffffffffffffffff,%rax
   6e9cf:	0f 84 ab 01 00 00                               	je     6eb80 <classic_compare_worker::encode+0x2e0>
   6e9d5:	0f 28 44 24 40                                  	movaps 0x40(%rsp),%xmm0
   6e9da:	0f 11 43 10                                     	movups %xmm0,0x10(%rbx)
   6e9de:	48 89 43 08                                     	mov    %rax,0x8(%rbx)
   6e9e2:	48 c7 03 01 00 00 00                            	movq   $0x1,(%rbx)
   6e9e9:	e9 61 05 00 00                                  	jmp    6ef4f <classic_compare_worker::encode+0x6af>
   6e9ee:	41 be 04 00 00 00                               	mov    $0x4,%r14d
   6e9f4:	31 d2                                           	xor    %edx,%edx
   6e9f6:	4d 85 ed                                        	test   %r13,%r13
   6e9f9:	48 89 54 24 30                                  	mov    %rdx,0x30(%rsp)
   6e9fe:	0f 84 20 04 00 00                               	je     6ee24 <classic_compare_worker::encode+0x584>
   6ea04:	49 83 fd 08                                     	cmp    $0x8,%r13
   6ea08:	72 1b                                           	jb     6ea25 <classic_compare_worker::encode+0x185>
   6ea0a:	4c 01 f3                                        	add    %r14,%rbx
   6ea0d:	4b 8d 04 2c                                     	lea    (%r12,%r13,1),%rax
   6ea11:	49 39 c6                                        	cmp    %rax,%r14
   6ea14:	0f 92 c0                                        	setb   %al
   6ea17:	49 39 dc                                        	cmp    %rbx,%r12
   6ea1a:	0f 92 c1                                        	setb   %cl
   6ea1d:	84 c8                                           	test   %cl,%al
   6ea1f:	0f 84 ae 01 00 00                               	je     6ebd3 <classic_compare_worker::encode+0x333>
   6ea25:	31 c0                                           	xor    %eax,%eax
   6ea27:	4c 89 ea                                        	mov    %r13,%rdx
   6ea2a:	48 89 c1                                        	mov    %rax,%rcx
   6ea2d:	48 83 e2 03                                     	and    $0x3,%rdx
   6ea31:	74 1e                                           	je     6ea51 <classic_compare_worker::encode+0x1b1>
   6ea33:	48 89 c1                                        	mov    %rax,%rcx
   6ea36:	66 2e 0f 1f 84 00 00 00 00 00                   	cs nopw 0x0(%rax,%rax,1)
   6ea40:	41 0f b6 34 0c                                  	movzbl (%r12,%rcx,1),%esi
   6ea45:	41 89 34 8e                                     	mov    %esi,(%r14,%rcx,4)
   6ea49:	48 ff c1                                        	inc    %rcx
   6ea4c:	48 ff ca                                        	dec    %rdx
   6ea4f:	75 ef                                           	jne    6ea40 <classic_compare_worker::encode+0x1a0>
   6ea51:	4c 29 e8                                        	sub    %r13,%rax
   6ea54:	48 83 f8 fc                                     	cmp    $0xfffffffffffffffc,%rax
   6ea58:	0f 87 c6 03 00 00                               	ja     6ee24 <classic_compare_worker::encode+0x584>
   6ea5e:	66 90                                           	xchg   %ax,%ax
   6ea60:	41 0f b6 04 0c                                  	movzbl (%r12,%rcx,1),%eax
   6ea65:	41 89 04 8e                                     	mov    %eax,(%r14,%rcx,4)
   6ea69:	41 0f b6 44 0c 01                               	movzbl 0x1(%r12,%rcx,1),%eax
   6ea6f:	41 89 44 8e 04                                  	mov    %eax,0x4(%r14,%rcx,4)
   6ea74:	41 0f b6 44 0c 02                               	movzbl 0x2(%r12,%rcx,1),%eax
   6ea7a:	41 89 44 8e 08                                  	mov    %eax,0x8(%r14,%rcx,4)
   6ea7f:	41 0f b6 44 0c 03                               	movzbl 0x3(%r12,%rcx,1),%eax
   6ea85:	41 89 44 8e 0c                                  	mov    %eax,0xc(%r14,%rcx,4)
   6ea8a:	48 83 c1 04                                     	add    $0x4,%rcx
   6ea8e:	49 39 cd                                        	cmp    %rcx,%r13
   6ea91:	75 cd                                           	jne    6ea60 <classic_compare_worker::encode+0x1c0>
   6ea93:	e9 8c 03 00 00                                  	jmp    6ee24 <classic_compare_worker::encode+0x584>
   6ea98:	41 be 04 00 00 00                               	mov    $0x4,%r14d
   6ea9e:	49 83 fd 02                                     	cmp    $0x2,%r13
   6eaa2:	0f 82 7c 03 00 00                               	jb     6ee24 <classic_compare_worker::encode+0x584>
   6eaa8:	48 b9 fc ff ff ff ff ff ff 7f                   	movabs $0x7ffffffffffffffc,%rcx
   6eab2:	48 8d 41 02                                     	lea    0x2(%rcx),%rax
   6eab6:	4c 21 e8                                        	and    %r13,%rax
   6eab9:	49 8d 75 fe                                     	lea    -0x2(%r13),%rsi
   6eabd:	48 83 fe 1e                                     	cmp    $0x1e,%rsi
   6eac1:	72 1f                                           	jb     6eae2 <classic_compare_worker::encode+0x242>
   6eac3:	49 8d 14 04                                     	lea    (%r12,%rax,1),%rdx
   6eac7:	49 39 d6                                        	cmp    %rdx,%r14
   6eaca:	0f 83 ff 02 00 00                               	jae    6edcf <classic_compare_worker::encode+0x52f>
   6ead0:	4d 01 ed                                        	add    %r13,%r13
   6ead3:	49 21 cd                                        	and    %rcx,%r13
   6ead6:	4d 01 f5                                        	add    %r14,%r13
   6ead9:	4d 39 ec                                        	cmp    %r13,%r12
   6eadc:	0f 83 ed 02 00 00                               	jae    6edcf <classic_compare_worker::encode+0x52f>
   6eae2:	31 d2                                           	xor    %edx,%edx
   6eae4:	4c 89 e1                                        	mov    %r12,%rcx
   6eae7:	48 8d 70 fe                                     	lea    -0x2(%rax),%rsi
   6eaeb:	89 f7                                           	mov    %esi,%edi
   6eaed:	f7 d7                                           	not    %edi
   6eaef:	40 f6 c7 06                                     	test   $0x6,%dil
   6eaf3:	74 39                                           	je     6eb2e <classic_compare_worker::encode+0x28e>
   6eaf5:	89 f7                                           	mov    %esi,%edi
   6eaf7:	d1 ef                                           	shr    $1,%edi
   6eaf9:	ff c7                                           	inc    %edi
   6eafb:	83 e7 03                                        	and    $0x3,%edi
   6eafe:	4d 8d 0c 96                                     	lea    (%r14,%rdx,4),%r9
   6eb02:	48 f7 df                                        	neg    %rdi
   6eb05:	45 31 c0                                        	xor    %r8d,%r8d
   6eb08:	45 31 d2                                        	xor    %r10d,%r10d
   6eb0b:	0f 1f 44 00 00                                  	nopl   0x0(%rax,%rax,1)
   6eb10:	46 0f b7 1c 01                                  	movzwl (%rcx,%r8,1),%r11d
   6eb15:	47 89 1c 41                                     	mov    %r11d,(%r9,%r8,2)
   6eb19:	49 ff ca                                        	dec    %r10
   6eb1c:	49 83 c0 02                                     	add    $0x2,%r8
   6eb20:	4c 39 d7                                        	cmp    %r10,%rdi
   6eb23:	75 eb                                           	jne    6eb10 <classic_compare_worker::encode+0x270>
   6eb25:	4c 29 d2                                        	sub    %r10,%rdx
   6eb28:	4c 29 c0                                        	sub    %r8,%rax
   6eb2b:	4c 01 c1                                        	add    %r8,%rcx
   6eb2e:	48 83 fe 06                                     	cmp    $0x6,%rsi
   6eb32:	0f 82 ec 02 00 00                               	jb     6ee24 <classic_compare_worker::encode+0x584>
   6eb38:	49 8d 14 96                                     	lea    (%r14,%rdx,4),%rdx
   6eb3c:	48 83 c2 0c                                     	add    $0xc,%rdx
   6eb40:	31 f6                                           	xor    %esi,%esi
   6eb42:	66 66 66 66 66 2e 0f 1f 84 00 00 00 00 00       	data16 data16 data16 data16 cs nopw 0x0(%rax,%rax,1)
   6eb50:	0f b7 3c 31                                     	movzwl (%rcx,%rsi,1),%edi
   6eb54:	89 7c 72 f4                                     	mov    %edi,-0xc(%rdx,%rsi,2)
   6eb58:	0f b7 7c 31 02                                  	movzwl 0x2(%rcx,%rsi,1),%edi
   6eb5d:	89 7c 72 f8                                     	mov    %edi,-0x8(%rdx,%rsi,2)
   6eb61:	0f b7 7c 31 04                                  	movzwl 0x4(%rcx,%rsi,1),%edi
   6eb66:	89 7c 72 fc                                     	mov    %edi,-0x4(%rdx,%rsi,2)
   6eb6a:	0f b7 7c 31 06                                  	movzwl 0x6(%rcx,%rsi,1),%edi
   6eb6f:	89 3c 72                                        	mov    %edi,(%rdx,%rsi,2)
   6eb72:	48 83 c6 08                                     	add    $0x8,%rsi
   6eb76:	48 39 f0                                        	cmp    %rsi,%rax
   6eb79:	75 d5                                           	jne    6eb50 <classic_compare_worker::encode+0x2b0>
   6eb7b:	e9 a4 02 00 00                                  	jmp    6ee24 <classic_compare_worker::encode+0x584>
   6eb80:	0f 28 44 24 40                                  	movaps 0x40(%rsp),%xmm0
   6eb85:	0f 29 84 24 a0 00 00 00                         	movaps %xmm0,0xa0(%rsp)
   6eb8d:	41 8b 8f c4 00 00 00                            	mov    0xc4(%r15),%ecx
   6eb94:	48 85 c9                                        	test   %rcx,%rcx
   6eb97:	0f 84 9b 04 00 00                               	je     6f038 <classic_compare_worker::encode+0x798>
   6eb9d:	41 8b 97 c0 00 00 00                            	mov    0xc0(%r15),%edx
   6eba4:	48 0f af d1                                     	imul   %rcx,%rdx
   6eba8:	41 0f b7 b7 c8 00 00 00                         	movzwl 0xc8(%r15),%esi
   6ebb0:	41 0f b6 87 ca 00 00 00                         	movzbl 0xca(%r15),%eax
   6ebb8:	c1 e8 03                                        	shr    $0x3,%eax
   6ebbb:	48 0f af c6                                     	imul   %rsi,%rax
   6ebbf:	48 0f af c2                                     	imul   %rdx,%rax
   6ebc3:	48 89 c2                                        	mov    %rax,%rdx
   6ebc6:	48 c1 ea 20                                     	shr    $0x20,%rdx
   6ebca:	74 65                                           	je     6ec31 <classic_compare_worker::encode+0x391>
   6ebcc:	31 d2                                           	xor    %edx,%edx
   6ebce:	48 f7 f1                                        	div    %rcx
   6ebd1:	eb 62                                           	jmp    6ec35 <classic_compare_worker::encode+0x395>
   6ebd3:	48 b8 ff ff ff ff ff ff ff 3f                   	movabs $0x3fffffffffffffff,%rax
   6ebdd:	48 83 c0 f9                                     	add    $0xfffffffffffffff9,%rax
   6ebe1:	4c 21 e8                                        	and    %r13,%rax
   6ebe4:	31 c9                                           	xor    %ecx,%ecx
   6ebe6:	66 0f ef c0                                     	pxor   %xmm0,%xmm0
   6ebea:	66 0f 1f 44 00 00                               	nopw   0x0(%rax,%rax,1)
   6ebf0:	66 41 0f 6e 0c 0c                               	movd   (%r12,%rcx,1),%xmm1
   6ebf6:	66 41 0f 6e 54 0c 04                            	movd   0x4(%r12,%rcx,1),%xmm2
   6ebfd:	66 0f 60 c8                                     	punpcklbw %xmm0,%xmm1
   6ec01:	66 0f 61 c8                                     	punpcklwd %xmm0,%xmm1
   6ec05:	66 0f 60 d0                                     	punpcklbw %xmm0,%xmm2
   6ec09:	66 0f 61 d0                                     	punpcklwd %xmm0,%xmm2
   6ec0d:	f3 41 0f 7f 0c 8e                               	movdqu %xmm1,(%r14,%rcx,4)
   6ec13:	f3 41 0f 7f 54 8e 10                            	movdqu %xmm2,0x10(%r14,%rcx,4)
   6ec1a:	48 83 c1 08                                     	add    $0x8,%rcx
   6ec1e:	48 39 c8                                        	cmp    %rcx,%rax
   6ec21:	75 cd                                           	jne    6ebf0 <classic_compare_worker::encode+0x350>
   6ec23:	49 39 c5                                        	cmp    %rax,%r13
   6ec26:	0f 84 f8 01 00 00                               	je     6ee24 <classic_compare_worker::encode+0x584>
   6ec2c:	e9 f6 fd ff ff                                  	jmp    6ea27 <classic_compare_worker::encode+0x187>
   6ec31:	31 d2                                           	xor    %edx,%edx
   6ec33:	f7 f1                                           	div    %ecx
   6ec35:	48 8d 8c 24 a0 00 00 00                         	lea    0xa0(%rsp),%rcx
   6ec3d:	48 89 4c 24 70                                  	mov    %rcx,0x70(%rsp)
   6ec42:	4c 89 64 24 78                                  	mov    %r12,0x78(%rsp)
   6ec47:	4c 89 ac 24 80 00 00 00                         	mov    %r13,0x80(%rsp)
   6ec4f:	48 89 84 24 88 00 00 00                         	mov    %rax,0x88(%rsp)
   6ec57:	49 8d 8f b0 00 00 00                            	lea    0xb0(%r15),%rcx
   6ec5e:	41 80 bf cb 00 00 00 00                         	cmpb   $0x0,0xcb(%r15)
   6ec66:	74 47                                           	je     6ecaf <classic_compare_worker::encode+0x40f>
   6ec68:	c7 44 24 18 00 00 00 00                         	movl   $0x0,0x18(%rsp)
   6ec70:	c7 44 24 2c 01 00 00 02                         	movl   $0x2000001,0x2c(%rsp)
   6ec78:	c7 44 24 20 00 00 00 00                         	movl   $0x0,0x20(%rsp)
   6ec80:	48 c7 04 24 00 00 00 00                         	movq   $0x0,(%rsp)
   6ec88:	48 c7 44 24 08 08 00 00 00                      	movq   $0x8,0x8(%rsp)
   6ec91:	48 c7 44 24 10 00 00 00 00                      	movq   $0x0,0x10(%rsp)
   6ec9a:	48 8d 7c 24 40                                  	lea    0x40(%rsp),%rdi
   6ec9f:	48 8d 74 24 70                                  	lea    0x70(%rsp),%rsi
   6eca4:	48 89 e2                                        	mov    %rsp,%rdx
   6eca7:	ff 15 1b 63 20 00                               	call   *0x20631b(%rip)        # 274fc8 <_DYNAMIC+0x418>
   6ecad:	eb 45                                           	jmp    6ecf4 <classic_compare_worker::encode+0x454>
   6ecaf:	c7 44 24 18 00 00 00 00                         	movl   $0x0,0x18(%rsp)
   6ecb7:	c7 44 24 2c 01 00 00 02                         	movl   $0x2000001,0x2c(%rsp)
   6ecbf:	c7 44 24 20 00 00 00 00                         	movl   $0x0,0x20(%rsp)
   6ecc7:	48 c7 04 24 00 00 00 00                         	movq   $0x0,(%rsp)
   6eccf:	48 c7 44 24 08 08 00 00 00                      	movq   $0x8,0x8(%rsp)
   6ecd8:	48 c7 44 24 10 00 00 00 00                      	movq   $0x0,0x10(%rsp)
   6ece1:	48 8d 7c 24 40                                  	lea    0x40(%rsp),%rdi
   6ece6:	48 8d 74 24 70                                  	lea    0x70(%rsp),%rsi
   6eceb:	48 89 e2                                        	mov    %rsp,%rdx
   6ecee:	ff 15 dc 62 20 00                               	call   *0x2062dc(%rip)        # 274fd0 <_DYNAMIC+0x420>
   6ecf4:	48 89 d9                                        	mov    %rbx,%rcx
   6ecf7:	48 83 7c 24 40 ff                               	cmpq   $0xffffffffffffffff,0x40(%rsp)
   6ecfd:	0f 84 ae 00 00 00                               	je     6edb1 <classic_compare_worker::encode+0x511>
   6ed03:	0f 10 44 24 40                                  	movups 0x40(%rsp),%xmm0
   6ed08:	f3 0f 6f 4c 24 50                               	movdqu 0x50(%rsp),%xmm1
   6ed0e:	f3 0f 6f 54 24 60                               	movdqu 0x60(%rsp),%xmm2
   6ed14:	66 0f 7f 54 24 20                               	movdqa %xmm2,0x20(%rsp)
   6ed1a:	66 0f 7f 4c 24 10                               	movdqa %xmm1,0x10(%rsp)
   6ed20:	0f 29 04 24                                     	movaps %xmm0,(%rsp)
   6ed24:	4c 89 b4 24 90 00 00 00                         	mov    %r14,0x90(%rsp)
   6ed2c:	48 8d 05 6d 39 00 00                            	lea    0x396d(%rip),%rax        # 726a0 <<emuella_j2k_core::J2kError as core::fmt::Debug>::fmt>
   6ed33:	48 89 84 24 98 00 00 00                         	mov    %rax,0x98(%rsp)
   6ed3b:	48 8d 35 a9 f3 f9 ff                            	lea    -0x60c57(%rip),%rsi        # e0eb <anon.b5c622d8c0861678d1a06bfbb6410fee.144.llvm.5232537515227879042>
   6ed42:	48 8d bc 24 b0 00 00 00                         	lea    0xb0(%rsp),%rdi
   6ed4a:	48 8d 94 24 90 00 00 00                         	lea    0x90(%rsp),%rdx
   6ed52:	ff 15 20 61 20 00                               	call   *0x206120(%rip)        # 274e78 <_DYNAMIC+0x2c8>
   6ed58:	48 8d 44 24 10                                  	lea    0x10(%rsp),%rax
   6ed5d:	48 8b 0c 24                                     	mov    (%rsp),%rcx
   6ed61:	48 be fc ff ff ff ff ff ff 7f                   	movabs $0x7ffffffffffffffc,%rsi
   6ed6b:	48 83 c6 04                                     	add    $0x4,%rsi
   6ed6f:	48 31 ce                                        	xor    %rcx,%rsi
   6ed72:	48 85 c9                                        	test   %rcx,%rcx
   6ed75:	ba 02 00 00 00                                  	mov    $0x2,%edx
   6ed7a:	48 0f 48 d6                                     	cmovs  %rsi,%rdx
   6ed7e:	48 83 fa 05                                     	cmp    $0x5,%rdx
   6ed82:	0f 87 51 02 00 00                               	ja     6efd9 <classic_compare_worker::encode+0x739>
   6ed88:	48 8d 35 f1 22 fa ff                            	lea    -0x5dd0f(%rip),%rsi        # 11080 <anon.ee651107ab5319c6bc273e1a29320aaf.108.llvm.14746981713632465754+0x40>
   6ed8f:	48 63 14 96                                     	movslq (%rsi,%rdx,4),%rdx
   6ed93:	48 01 f2                                        	add    %rsi,%rdx
   6ed96:	48 89 de                                        	mov    %rbx,%rsi
   6ed99:	ff e2                                           	jmp    *%rdx
   6ed9b:	48 83 7c 24 18 00                               	cmpq   $0x0,0x18(%rsp)
   6eda1:	0f 84 55 02 00 00                               	je     6effc <classic_compare_worker::encode+0x75c>
   6eda7:	48 8d 44 24 20                                  	lea    0x20(%rsp),%rax
   6edac:	e9 3f 02 00 00                                  	jmp    6eff0 <classic_compare_worker::encode+0x750>
   6edb1:	48 8b 44 24 58                                  	mov    0x58(%rsp),%rax
   6edb6:	48 89 41 18                                     	mov    %rax,0x18(%rcx)
   6edba:	0f 10 44 24 48                                  	movups 0x48(%rsp),%xmm0
   6edbf:	0f 11 41 08                                     	movups %xmm0,0x8(%rcx)
   6edc3:	48 c7 01 00 00 00 00                            	movq   $0x0,(%rcx)
   6edca:	e9 80 01 00 00                                  	jmp    6ef4f <classic_compare_worker::encode+0x6af>
   6edcf:	48 d1 ee                                        	shr    $1,%rsi
   6edd2:	48 ff c6                                        	inc    %rsi
   6edd5:	48 89 f2                                        	mov    %rsi,%rdx
   6edd8:	48 83 e2 f8                                     	and    $0xfffffffffffffff8,%rdx
   6eddc:	48 29 d0                                        	sub    %rdx,%rax
   6eddf:	48 29 d0                                        	sub    %rdx,%rax
   6ede2:	49 8d 0c 54                                     	lea    (%r12,%rdx,2),%rcx
   6ede6:	31 ff                                           	xor    %edi,%edi
   6ede8:	66 0f ef c0                                     	pxor   %xmm0,%xmm0
   6edec:	0f 1f 40 00                                     	nopl   0x0(%rax)
   6edf0:	f3 41 0f 7e 0c 7c                               	movq   (%r12,%rdi,2),%xmm1
   6edf6:	f3 41 0f 7e 54 7c 08                            	movq   0x8(%r12,%rdi,2),%xmm2
   6edfd:	66 0f 61 c8                                     	punpcklwd %xmm0,%xmm1
   6ee01:	66 0f 61 d0                                     	punpcklwd %xmm0,%xmm2
   6ee05:	f3 41 0f 7f 0c be                               	movdqu %xmm1,(%r14,%rdi,4)
   6ee0b:	f3 41 0f 7f 54 be 10                            	movdqu %xmm2,0x10(%r14,%rdi,4)
   6ee12:	48 83 c7 08                                     	add    $0x8,%rdi
   6ee16:	48 39 fa                                        	cmp    %rdi,%rdx
   6ee19:	75 d5                                           	jne    6edf0 <classic_compare_worker::encode+0x550>
   6ee1b:	48 39 d6                                        	cmp    %rdx,%rsi
   6ee1e:	0f 85 c3 fc ff ff                               	jne    6eae7 <classic_compare_worker::encode+0x247>
   6ee24:	48 c7 44 24 40 00 00 00 00                      	movq   $0x0,0x40(%rsp)
   6ee2d:	48 c7 04 24 00 00 00 00                         	movq   $0x0,(%rsp)
   6ee35:	41 8b b7 c0 00 00 00                            	mov    0xc0(%r15),%esi
   6ee3c:	41 8b 97 c4 00 00 00                            	mov    0xc4(%r15),%edx
   6ee43:	41 0f b7 8f c8 00 00 00                         	movzwl 0xc8(%r15),%ecx
   6ee4b:	31 c0                                           	xor    %eax,%eax
   6ee4d:	83 f9 03                                        	cmp    $0x3,%ecx
   6ee50:	0f 94 c0                                        	sete   %al
   6ee53:	45 0f b6 97 cc 00 00 00                         	movzbl 0xcc(%r15),%r10d
   6ee5b:	45 0f b6 9f cb 00 00 00                         	movzbl 0xcb(%r15),%r11d
   6ee63:	48 89 e3                                        	mov    %rsp,%rbx
   6ee66:	4c 8d 64 24 40                                  	lea    0x40(%rsp),%r12
   6ee6b:	f2 0f 10 05 85 21 fa ff                         	movsd  -0x5de7b(%rip),%xmm0        # 10ff8 <anon.163a7daa153173e3324b1e1e59ef13e5.3.llvm.12432551406381563140+0x398>
   6ee73:	4c 89 f7                                        	mov    %r14,%rdi
   6ee76:	41 89 e8                                        	mov    %ebp,%r8d
   6ee79:	41 b9 02 00 00 00                               	mov    $0x2,%r9d
   6ee7f:	53                                              	push   %rbx
   6ee80:	41 54                                           	push   %r12
   6ee82:	50                                              	push   %rax
   6ee83:	41 53                                           	push   %r11
   6ee85:	41 52                                           	push   %r10
   6ee87:	6a 01                                           	push   $0x1
   6ee89:	ff 15 49 61 20 00                               	call   *0x206149(%rip)        # 274fd8 <_DYNAMIC+0x428>
   6ee8f:	48 83 c4 30                                     	add    $0x30,%rsp
   6ee93:	85 c0                                           	test   %eax,%eax
   6ee95:	74 58                                           	je     6eeef <classic_compare_worker::encode+0x64f>
   6ee97:	4c 8b 24 24                                     	mov    (%rsp),%r12
   6ee9b:	48 8b 5c 24 40                                  	mov    0x40(%rsp),%rbx
   6eea0:	4d 3b a7 b8 00 00 00                            	cmp    0xb8(%r15),%r12
   6eea7:	0f 86 b4 00 00 00                               	jbe    6ef61 <classic_compare_worker::encode+0x6c1>
   6eead:	48 89 df                                        	mov    %rbx,%rdi
   6eeb0:	ff 15 0a 61 20 00                               	call   *0x20610a(%rip)        # 274fc0 <_DYNAMIC+0x410>
   6eeb6:	41 bc 15 00 00 00                               	mov    $0x15,%r12d
   6eebc:	bf 15 00 00 00                                  	mov    $0x15,%edi
   6eec1:	ff 15 f1 5e 20 00                               	call   *0x205ef1(%rip)        # 274db8 <malloc@GLIBC_2.2.5>
   6eec7:	48 85 c0                                        	test   %rax,%rax
   6eeca:	0f 84 50 01 00 00                               	je     6f020 <classic_compare_worker::encode+0x780>
   6eed0:	0f 10 05 a8 24 fa ff                            	movups -0x5db58(%rip),%xmm0        # 1137f <anon.ee651107ab5319c6bc273e1a29320aaf.108.llvm.14746981713632465754+0x33f>
   6eed7:	0f 11 00                                        	movups %xmm0,(%rax)
   6eeda:	48 b9 65 78 63 65 65 64 65 64                   	movabs $0x6465646565637865,%rcx
   6eee4:	48 89 48 0d                                     	mov    %rcx,0xd(%rax)
   6eee8:	b9 15 00 00 00                                  	mov    $0x15,%ecx
   6eeed:	eb 37                                           	jmp    6ef26 <classic_compare_worker::encode+0x686>
   6eeef:	41 bc 16 00 00 00                               	mov    $0x16,%r12d
   6eef5:	bf 16 00 00 00                                  	mov    $0x16,%edi
   6eefa:	ff 15 b8 5e 20 00                               	call   *0x205eb8(%rip)        # 274db8 <malloc@GLIBC_2.2.5>
   6ef00:	48 85 c0                                        	test   %rax,%rax
   6ef03:	0f 84 17 01 00 00                               	je     6f020 <classic_compare_worker::encode+0x780>
   6ef09:	0f 10 05 59 26 fa ff                            	movups -0x5d9a7(%rip),%xmm0        # 11569 <anon.ee651107ab5319c6bc273e1a29320aaf.108.llvm.14746981713632465754+0x529>
   6ef10:	0f 11 00                                        	movups %xmm0,(%rax)
   6ef13:	48 b9 65 20 66 61 69 6c 65 64                   	movabs $0x64656c6961662065,%rcx
   6ef1d:	48 89 48 0e                                     	mov    %rcx,0xe(%rax)
   6ef21:	b9 16 00 00 00                                  	mov    $0x16,%ecx
   6ef26:	48 8b 54 24 38                                  	mov    0x38(%rsp),%rdx
   6ef2b:	48 89 4a 08                                     	mov    %rcx,0x8(%rdx)
   6ef2f:	48 89 42 10                                     	mov    %rax,0x10(%rdx)
   6ef33:	48 89 4a 18                                     	mov    %rcx,0x18(%rdx)
   6ef37:	48 c7 02 01 00 00 00                            	movq   $0x1,(%rdx)
   6ef3e:	48 83 7c 24 30 00                               	cmpq   $0x0,0x30(%rsp)
   6ef44:	74 09                                           	je     6ef4f <classic_compare_worker::encode+0x6af>
   6ef46:	4c 89 f7                                        	mov    %r14,%rdi
   6ef49:	ff 15 79 5e 20 00                               	call   *0x205e79(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   6ef4f:	48 81 c4 c8 00 00 00                            	add    $0xc8,%rsp
   6ef56:	5b                                              	pop    %rbx
   6ef57:	41 5c                                           	pop    %r12
   6ef59:	41 5d                                           	pop    %r13
   6ef5b:	41 5e                                           	pop    %r14
   6ef5d:	41 5f                                           	pop    %r15
   6ef5f:	5d                                              	pop    %rbp
   6ef60:	c3                                              	ret
   6ef61:	4d 85 e4                                        	test   %r12,%r12
   6ef64:	79 0d                                           	jns    6ef73 <classic_compare_worker::encode+0x6d3>
   6ef66:	31 ff                                           	xor    %edi,%edi
   6ef68:	4c 89 e6                                        	mov    %r12,%rsi
   6ef6b:	ff 15 8f 5e 20 00                               	call   *0x205e8f(%rip)        # 274e00 <_DYNAMIC+0x250>
   6ef71:	0f 0b                                           	ud2
   6ef73:	74 26                                           	je     6ef9b <classic_compare_worker::encode+0x6fb>
   6ef75:	4c 89 e7                                        	mov    %r12,%rdi
   6ef78:	ff 15 3a 5e 20 00                               	call   *0x205e3a(%rip)        # 274db8 <malloc@GLIBC_2.2.5>
   6ef7e:	48 85 c0                                        	test   %rax,%rax
   6ef81:	0f 84 99 00 00 00                               	je     6f020 <classic_compare_worker::encode+0x780>
   6ef87:	49 89 c7                                        	mov    %rax,%r15
   6ef8a:	48 89 c7                                        	mov    %rax,%rdi
   6ef8d:	48 89 de                                        	mov    %rbx,%rsi
   6ef90:	4c 89 e2                                        	mov    %r12,%rdx
   6ef93:	ff 15 3f 5e 20 00                               	call   *0x205e3f(%rip)        # 274dd8 <memcpy@GLIBC_2.14>
   6ef99:	eb 06                                           	jmp    6efa1 <classic_compare_worker::encode+0x701>
   6ef9b:	41 bf 01 00 00 00                               	mov    $0x1,%r15d
   6efa1:	48 89 df                                        	mov    %rbx,%rdi
   6efa4:	ff 15 16 60 20 00                               	call   *0x206016(%rip)        # 274fc0 <_DYNAMIC+0x410>
   6efaa:	48 8b 44 24 38                                  	mov    0x38(%rsp),%rax
   6efaf:	4c 89 60 08                                     	mov    %r12,0x8(%rax)
   6efb3:	4c 89 78 10                                     	mov    %r15,0x10(%rax)
   6efb7:	4c 89 60 18                                     	mov    %r12,0x18(%rax)
   6efbb:	48 c7 00 00 00 00 00                            	movq   $0x0,(%rax)
   6efc2:	48 83 7c 24 30 00                               	cmpq   $0x0,0x30(%rsp)
   6efc8:	74 85                                           	je     6ef4f <classic_compare_worker::encode+0x6af>
   6efca:	e9 77 ff ff ff                                  	jmp    6ef46 <classic_compare_worker::encode+0x6a6>
   6efcf:	48 83 7c 24 08 00                               	cmpq   $0x0,0x8(%rsp)
   6efd5:	75 19                                           	jne    6eff0 <classic_compare_worker::encode+0x750>
   6efd7:	eb 23                                           	jmp    6effc <classic_compare_worker::encode+0x75c>
   6efd9:	48 83 7c 24 08 00                               	cmpq   $0x0,0x8(%rsp)
   6efdf:	48 89 de                                        	mov    %rbx,%rsi
   6efe2:	75 0c                                           	jne    6eff0 <classic_compare_worker::encode+0x750>
   6efe4:	eb 16                                           	jmp    6effc <classic_compare_worker::encode+0x75c>
   6efe6:	48 85 c9                                        	test   %rcx,%rcx
   6efe9:	74 11                                           	je     6effc <classic_compare_worker::encode+0x75c>
   6efeb:	48 8d 44 24 08                                  	lea    0x8(%rsp),%rax
   6eff0:	48 8b 38                                        	mov    (%rax),%rdi
   6eff3:	ff 15 cf 5d 20 00                               	call   *0x205dcf(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   6eff9:	48 89 de                                        	mov    %rbx,%rsi
   6effc:	48 8b 84 24 c0 00 00 00                         	mov    0xc0(%rsp),%rax
   6f004:	48 89 46 18                                     	mov    %rax,0x18(%rsi)
   6f008:	0f 10 84 24 b0 00 00 00                         	movups 0xb0(%rsp),%xmm0
   6f010:	0f 11 46 08                                     	movups %xmm0,0x8(%rsi)
   6f014:	48 c7 06 01 00 00 00                            	movq   $0x1,(%rsi)
   6f01b:	e9 2f ff ff ff                                  	jmp    6ef4f <classic_compare_worker::encode+0x6af>
   6f020:	bf 01 00 00 00                                  	mov    $0x1,%edi
   6f025:	e9 3e ff ff ff                                  	jmp    6ef68 <classic_compare_worker::encode+0x6c8>
   6f02a:	bf 04 00 00 00                                  	mov    $0x4,%edi
   6f02f:	48 89 de                                        	mov    %rbx,%rsi
   6f032:	ff 15 c8 5d 20 00                               	call   *0x205dc8(%rip)        # 274e00 <_DYNAMIC+0x250>
   6f038:	48 8d 3d d1 e5 1f 00                            	lea    0x1fe5d1(%rip),%rdi        # 26d610 <__frame_dummy_init_array_entry+0x290>
   6f03f:	ff 15 9b 5f 20 00                               	call   *0x205f9b(%rip)        # 274fe0 <_DYNAMIC+0x430>
   6f045:	eb 13                                           	jmp    6f05a <classic_compare_worker::encode+0x7ba>
   6f047:	48 89 c3                                        	mov    %rax,%rbx
   6f04a:	48 89 e7                                        	mov    %rsp,%rdi
   6f04d:	e8 ae 59 ff ff                                  	call   64a00 <core::ptr::drop_glue::<emuella_j2k_core::J2kError>>
   6f052:	48 89 df                                        	mov    %rbx,%rdi
   6f055:	e8 46 d1 1f 00                                  	call   26c1a0 <_Unwind_Resume@plt>
   6f05a:	48 89 c3                                        	mov    %rax,%rbx
   6f05d:	48 89 e7                                        	mov    %rsp,%rdi
   6f060:	e8 1b 59 ff ff                                  	call   64980 <core::ptr::drop_glue::<emuella_j2k_core::EncodeOptions>>
   6f065:	48 89 df                                        	mov    %rbx,%rdi
   6f068:	e8 33 d1 1f 00                                  	call   26c1a0 <_Unwind_Resume@plt>
   6f06d:	48 89 c3                                        	mov    %rax,%rbx
   6f070:	48 83 7c 24 30 00                               	cmpq   $0x0,0x30(%rsp)
   6f076:	74 09                                           	je     6f081 <classic_compare_worker::encode+0x7e1>
   6f078:	4c 89 f7                                        	mov    %r14,%rdi
   6f07b:	ff 15 47 5d 20 00                               	call   *0x205d47(%rip)        # 274dc8 <free@GLIBC_2.2.5>
   6f081:	48 89 df                                        	mov    %rbx,%rdi
   6f084:	e8 17 d1 1f 00                                  	call   26c1a0 <_Unwind_Resume@plt>
