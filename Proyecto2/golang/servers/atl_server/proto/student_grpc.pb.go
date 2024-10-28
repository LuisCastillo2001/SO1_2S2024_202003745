// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.5.1
// - protoc             v3.12.4
// source: student.proto

package confproto

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.64.0 or later.
const _ = grpc.SupportPackageIsVersion9

const (
	Student_GetStudent_FullMethodName = "/confproto.Student/GetStudent"
)

// StudentClient is the client API for Student service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type StudentClient interface {
	GetStudent(ctx context.Context, in *StudentRequest, opts ...grpc.CallOption) (*StudentResponse, error)
}

type studentClient struct {
	cc grpc.ClientConnInterface
}

func NewStudentClient(cc grpc.ClientConnInterface) StudentClient {
	return &studentClient{cc}
}

func (c *studentClient) GetStudent(ctx context.Context, in *StudentRequest, opts ...grpc.CallOption) (*StudentResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(StudentResponse)
	err := c.cc.Invoke(ctx, Student_GetStudent_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// StudentServer is the server API for Student service.
// All implementations must embed UnimplementedStudentServer
// for forward compatibility.
type StudentServer interface {
	GetStudent(context.Context, *StudentRequest) (*StudentResponse, error)
	mustEmbedUnimplementedStudentServer()
}

// UnimplementedStudentServer must be embedded to have
// forward compatible implementations.
//
// NOTE: this should be embedded by value instead of pointer to avoid a nil
// pointer dereference when methods are called.
type UnimplementedStudentServer struct{}

func (UnimplementedStudentServer) GetStudent(context.Context, *StudentRequest) (*StudentResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetStudent not implemented")
}
func (UnimplementedStudentServer) mustEmbedUnimplementedStudentServer() {}
func (UnimplementedStudentServer) testEmbeddedByValue()                 {}

// UnsafeStudentServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to StudentServer will
// result in compilation errors.
type UnsafeStudentServer interface {
	mustEmbedUnimplementedStudentServer()
}

func RegisterStudentServer(s grpc.ServiceRegistrar, srv StudentServer) {
	// If the following call pancis, it indicates UnimplementedStudentServer was
	// embedded by pointer and is nil.  This will cause panics if an
	// unimplemented method is ever invoked, so we test this at initialization
	// time to prevent it from happening at runtime later due to I/O.
	if t, ok := srv.(interface{ testEmbeddedByValue() }); ok {
		t.testEmbeddedByValue()
	}
	s.RegisterService(&Student_ServiceDesc, srv)
}

func _Student_GetStudent_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(StudentRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(StudentServer).GetStudent(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: Student_GetStudent_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(StudentServer).GetStudent(ctx, req.(*StudentRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// Student_ServiceDesc is the grpc.ServiceDesc for Student service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var Student_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "confproto.Student",
	HandlerType: (*StudentServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "GetStudent",
			Handler:    _Student_GetStudent_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "student.proto",
}
