// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'agent_loop.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$AgentEvent {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String text) content,
    required TResult Function(String text) reasoning,
    required TResult Function(String url) image,
    required TResult Function(List<ToolCallInfo> calls) toolCalls,
    required TResult Function(List<ToolResultInfo> results) toolResults,
    required TResult Function(
      int promptTokens,
      int completionTokens,
      int totalTokens,
    )
    usage,
    required TResult Function() done,
    required TResult Function(String message) error,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String text)? content,
    TResult? Function(String text)? reasoning,
    TResult? Function(String url)? image,
    TResult? Function(List<ToolCallInfo> calls)? toolCalls,
    TResult? Function(List<ToolResultInfo> results)? toolResults,
    TResult? Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult? Function()? done,
    TResult? Function(String message)? error,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String text)? content,
    TResult Function(String text)? reasoning,
    TResult Function(String url)? image,
    TResult Function(List<ToolCallInfo> calls)? toolCalls,
    TResult Function(List<ToolResultInfo> results)? toolResults,
    TResult Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult Function()? done,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AgentEvent_Content value) content,
    required TResult Function(AgentEvent_Reasoning value) reasoning,
    required TResult Function(AgentEvent_Image value) image,
    required TResult Function(AgentEvent_ToolCalls value) toolCalls,
    required TResult Function(AgentEvent_ToolResults value) toolResults,
    required TResult Function(AgentEvent_Usage value) usage,
    required TResult Function(AgentEvent_Done value) done,
    required TResult Function(AgentEvent_Error value) error,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AgentEvent_Content value)? content,
    TResult? Function(AgentEvent_Reasoning value)? reasoning,
    TResult? Function(AgentEvent_Image value)? image,
    TResult? Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult? Function(AgentEvent_ToolResults value)? toolResults,
    TResult? Function(AgentEvent_Usage value)? usage,
    TResult? Function(AgentEvent_Done value)? done,
    TResult? Function(AgentEvent_Error value)? error,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AgentEvent_Content value)? content,
    TResult Function(AgentEvent_Reasoning value)? reasoning,
    TResult Function(AgentEvent_Image value)? image,
    TResult Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult Function(AgentEvent_ToolResults value)? toolResults,
    TResult Function(AgentEvent_Usage value)? usage,
    TResult Function(AgentEvent_Done value)? done,
    TResult Function(AgentEvent_Error value)? error,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $AgentEventCopyWith<$Res> {
  factory $AgentEventCopyWith(
    AgentEvent value,
    $Res Function(AgentEvent) then,
  ) = _$AgentEventCopyWithImpl<$Res, AgentEvent>;
}

/// @nodoc
class _$AgentEventCopyWithImpl<$Res, $Val extends AgentEvent>
    implements $AgentEventCopyWith<$Res> {
  _$AgentEventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;
}

/// @nodoc
abstract class _$$AgentEvent_ContentImplCopyWith<$Res> {
  factory _$$AgentEvent_ContentImplCopyWith(
    _$AgentEvent_ContentImpl value,
    $Res Function(_$AgentEvent_ContentImpl) then,
  ) = __$$AgentEvent_ContentImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String text});
}

/// @nodoc
class __$$AgentEvent_ContentImplCopyWithImpl<$Res>
    extends _$AgentEventCopyWithImpl<$Res, _$AgentEvent_ContentImpl>
    implements _$$AgentEvent_ContentImplCopyWith<$Res> {
  __$$AgentEvent_ContentImplCopyWithImpl(
    _$AgentEvent_ContentImpl _value,
    $Res Function(_$AgentEvent_ContentImpl) _then,
  ) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? text = null}) {
    return _then(
      _$AgentEvent_ContentImpl(
        text: null == text
            ? _value.text
            : text // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$AgentEvent_ContentImpl extends AgentEvent_Content {
  const _$AgentEvent_ContentImpl({required this.text}) : super._();

  @override
  final String text;

  @override
  String toString() {
    return 'AgentEvent.content(text: $text)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AgentEvent_ContentImpl &&
            (identical(other.text, text) || other.text == text));
  }

  @override
  int get hashCode => Object.hash(runtimeType, text);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AgentEvent_ContentImplCopyWith<_$AgentEvent_ContentImpl> get copyWith =>
      __$$AgentEvent_ContentImplCopyWithImpl<_$AgentEvent_ContentImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String text) content,
    required TResult Function(String text) reasoning,
    required TResult Function(String url) image,
    required TResult Function(List<ToolCallInfo> calls) toolCalls,
    required TResult Function(List<ToolResultInfo> results) toolResults,
    required TResult Function(
      int promptTokens,
      int completionTokens,
      int totalTokens,
    )
    usage,
    required TResult Function() done,
    required TResult Function(String message) error,
  }) {
    return content(text);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String text)? content,
    TResult? Function(String text)? reasoning,
    TResult? Function(String url)? image,
    TResult? Function(List<ToolCallInfo> calls)? toolCalls,
    TResult? Function(List<ToolResultInfo> results)? toolResults,
    TResult? Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult? Function()? done,
    TResult? Function(String message)? error,
  }) {
    return content?.call(text);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String text)? content,
    TResult Function(String text)? reasoning,
    TResult Function(String url)? image,
    TResult Function(List<ToolCallInfo> calls)? toolCalls,
    TResult Function(List<ToolResultInfo> results)? toolResults,
    TResult Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult Function()? done,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (content != null) {
      return content(text);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AgentEvent_Content value) content,
    required TResult Function(AgentEvent_Reasoning value) reasoning,
    required TResult Function(AgentEvent_Image value) image,
    required TResult Function(AgentEvent_ToolCalls value) toolCalls,
    required TResult Function(AgentEvent_ToolResults value) toolResults,
    required TResult Function(AgentEvent_Usage value) usage,
    required TResult Function(AgentEvent_Done value) done,
    required TResult Function(AgentEvent_Error value) error,
  }) {
    return content(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AgentEvent_Content value)? content,
    TResult? Function(AgentEvent_Reasoning value)? reasoning,
    TResult? Function(AgentEvent_Image value)? image,
    TResult? Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult? Function(AgentEvent_ToolResults value)? toolResults,
    TResult? Function(AgentEvent_Usage value)? usage,
    TResult? Function(AgentEvent_Done value)? done,
    TResult? Function(AgentEvent_Error value)? error,
  }) {
    return content?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AgentEvent_Content value)? content,
    TResult Function(AgentEvent_Reasoning value)? reasoning,
    TResult Function(AgentEvent_Image value)? image,
    TResult Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult Function(AgentEvent_ToolResults value)? toolResults,
    TResult Function(AgentEvent_Usage value)? usage,
    TResult Function(AgentEvent_Done value)? done,
    TResult Function(AgentEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (content != null) {
      return content(this);
    }
    return orElse();
  }
}

abstract class AgentEvent_Content extends AgentEvent {
  const factory AgentEvent_Content({required final String text}) =
      _$AgentEvent_ContentImpl;
  const AgentEvent_Content._() : super._();

  String get text;
  @JsonKey(ignore: true)
  _$$AgentEvent_ContentImplCopyWith<_$AgentEvent_ContentImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AgentEvent_ReasoningImplCopyWith<$Res> {
  factory _$$AgentEvent_ReasoningImplCopyWith(
    _$AgentEvent_ReasoningImpl value,
    $Res Function(_$AgentEvent_ReasoningImpl) then,
  ) = __$$AgentEvent_ReasoningImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String text});
}

/// @nodoc
class __$$AgentEvent_ReasoningImplCopyWithImpl<$Res>
    extends _$AgentEventCopyWithImpl<$Res, _$AgentEvent_ReasoningImpl>
    implements _$$AgentEvent_ReasoningImplCopyWith<$Res> {
  __$$AgentEvent_ReasoningImplCopyWithImpl(
    _$AgentEvent_ReasoningImpl _value,
    $Res Function(_$AgentEvent_ReasoningImpl) _then,
  ) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? text = null}) {
    return _then(
      _$AgentEvent_ReasoningImpl(
        text: null == text
            ? _value.text
            : text // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$AgentEvent_ReasoningImpl extends AgentEvent_Reasoning {
  const _$AgentEvent_ReasoningImpl({required this.text}) : super._();

  @override
  final String text;

  @override
  String toString() {
    return 'AgentEvent.reasoning(text: $text)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AgentEvent_ReasoningImpl &&
            (identical(other.text, text) || other.text == text));
  }

  @override
  int get hashCode => Object.hash(runtimeType, text);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AgentEvent_ReasoningImplCopyWith<_$AgentEvent_ReasoningImpl>
  get copyWith =>
      __$$AgentEvent_ReasoningImplCopyWithImpl<_$AgentEvent_ReasoningImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String text) content,
    required TResult Function(String text) reasoning,
    required TResult Function(String url) image,
    required TResult Function(List<ToolCallInfo> calls) toolCalls,
    required TResult Function(List<ToolResultInfo> results) toolResults,
    required TResult Function(
      int promptTokens,
      int completionTokens,
      int totalTokens,
    )
    usage,
    required TResult Function() done,
    required TResult Function(String message) error,
  }) {
    return reasoning(text);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String text)? content,
    TResult? Function(String text)? reasoning,
    TResult? Function(String url)? image,
    TResult? Function(List<ToolCallInfo> calls)? toolCalls,
    TResult? Function(List<ToolResultInfo> results)? toolResults,
    TResult? Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult? Function()? done,
    TResult? Function(String message)? error,
  }) {
    return reasoning?.call(text);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String text)? content,
    TResult Function(String text)? reasoning,
    TResult Function(String url)? image,
    TResult Function(List<ToolCallInfo> calls)? toolCalls,
    TResult Function(List<ToolResultInfo> results)? toolResults,
    TResult Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult Function()? done,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (reasoning != null) {
      return reasoning(text);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AgentEvent_Content value) content,
    required TResult Function(AgentEvent_Reasoning value) reasoning,
    required TResult Function(AgentEvent_Image value) image,
    required TResult Function(AgentEvent_ToolCalls value) toolCalls,
    required TResult Function(AgentEvent_ToolResults value) toolResults,
    required TResult Function(AgentEvent_Usage value) usage,
    required TResult Function(AgentEvent_Done value) done,
    required TResult Function(AgentEvent_Error value) error,
  }) {
    return reasoning(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AgentEvent_Content value)? content,
    TResult? Function(AgentEvent_Reasoning value)? reasoning,
    TResult? Function(AgentEvent_Image value)? image,
    TResult? Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult? Function(AgentEvent_ToolResults value)? toolResults,
    TResult? Function(AgentEvent_Usage value)? usage,
    TResult? Function(AgentEvent_Done value)? done,
    TResult? Function(AgentEvent_Error value)? error,
  }) {
    return reasoning?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AgentEvent_Content value)? content,
    TResult Function(AgentEvent_Reasoning value)? reasoning,
    TResult Function(AgentEvent_Image value)? image,
    TResult Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult Function(AgentEvent_ToolResults value)? toolResults,
    TResult Function(AgentEvent_Usage value)? usage,
    TResult Function(AgentEvent_Done value)? done,
    TResult Function(AgentEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (reasoning != null) {
      return reasoning(this);
    }
    return orElse();
  }
}

abstract class AgentEvent_Reasoning extends AgentEvent {
  const factory AgentEvent_Reasoning({required final String text}) =
      _$AgentEvent_ReasoningImpl;
  const AgentEvent_Reasoning._() : super._();

  String get text;
  @JsonKey(ignore: true)
  _$$AgentEvent_ReasoningImplCopyWith<_$AgentEvent_ReasoningImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AgentEvent_ImageImplCopyWith<$Res> {
  factory _$$AgentEvent_ImageImplCopyWith(
    _$AgentEvent_ImageImpl value,
    $Res Function(_$AgentEvent_ImageImpl) then,
  ) = __$$AgentEvent_ImageImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String url});
}

/// @nodoc
class __$$AgentEvent_ImageImplCopyWithImpl<$Res>
    extends _$AgentEventCopyWithImpl<$Res, _$AgentEvent_ImageImpl>
    implements _$$AgentEvent_ImageImplCopyWith<$Res> {
  __$$AgentEvent_ImageImplCopyWithImpl(
    _$AgentEvent_ImageImpl _value,
    $Res Function(_$AgentEvent_ImageImpl) _then,
  ) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? url = null}) {
    return _then(
      _$AgentEvent_ImageImpl(
        url: null == url
            ? _value.url
            : url // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$AgentEvent_ImageImpl extends AgentEvent_Image {
  const _$AgentEvent_ImageImpl({required this.url}) : super._();

  @override
  final String url;

  @override
  String toString() {
    return 'AgentEvent.image(url: $url)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AgentEvent_ImageImpl &&
            (identical(other.url, url) || other.url == url));
  }

  @override
  int get hashCode => Object.hash(runtimeType, url);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AgentEvent_ImageImplCopyWith<_$AgentEvent_ImageImpl> get copyWith =>
      __$$AgentEvent_ImageImplCopyWithImpl<_$AgentEvent_ImageImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String text) content,
    required TResult Function(String text) reasoning,
    required TResult Function(String url) image,
    required TResult Function(List<ToolCallInfo> calls) toolCalls,
    required TResult Function(List<ToolResultInfo> results) toolResults,
    required TResult Function(
      int promptTokens,
      int completionTokens,
      int totalTokens,
    )
    usage,
    required TResult Function() done,
    required TResult Function(String message) error,
  }) {
    return image(url);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String text)? content,
    TResult? Function(String text)? reasoning,
    TResult? Function(String url)? image,
    TResult? Function(List<ToolCallInfo> calls)? toolCalls,
    TResult? Function(List<ToolResultInfo> results)? toolResults,
    TResult? Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult? Function()? done,
    TResult? Function(String message)? error,
  }) {
    return image?.call(url);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String text)? content,
    TResult Function(String text)? reasoning,
    TResult Function(String url)? image,
    TResult Function(List<ToolCallInfo> calls)? toolCalls,
    TResult Function(List<ToolResultInfo> results)? toolResults,
    TResult Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult Function()? done,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (image != null) {
      return image(url);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AgentEvent_Content value) content,
    required TResult Function(AgentEvent_Reasoning value) reasoning,
    required TResult Function(AgentEvent_Image value) image,
    required TResult Function(AgentEvent_ToolCalls value) toolCalls,
    required TResult Function(AgentEvent_ToolResults value) toolResults,
    required TResult Function(AgentEvent_Usage value) usage,
    required TResult Function(AgentEvent_Done value) done,
    required TResult Function(AgentEvent_Error value) error,
  }) {
    return image(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AgentEvent_Content value)? content,
    TResult? Function(AgentEvent_Reasoning value)? reasoning,
    TResult? Function(AgentEvent_Image value)? image,
    TResult? Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult? Function(AgentEvent_ToolResults value)? toolResults,
    TResult? Function(AgentEvent_Usage value)? usage,
    TResult? Function(AgentEvent_Done value)? done,
    TResult? Function(AgentEvent_Error value)? error,
  }) {
    return image?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AgentEvent_Content value)? content,
    TResult Function(AgentEvent_Reasoning value)? reasoning,
    TResult Function(AgentEvent_Image value)? image,
    TResult Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult Function(AgentEvent_ToolResults value)? toolResults,
    TResult Function(AgentEvent_Usage value)? usage,
    TResult Function(AgentEvent_Done value)? done,
    TResult Function(AgentEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (image != null) {
      return image(this);
    }
    return orElse();
  }
}

abstract class AgentEvent_Image extends AgentEvent {
  const factory AgentEvent_Image({required final String url}) =
      _$AgentEvent_ImageImpl;
  const AgentEvent_Image._() : super._();

  String get url;
  @JsonKey(ignore: true)
  _$$AgentEvent_ImageImplCopyWith<_$AgentEvent_ImageImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AgentEvent_ToolCallsImplCopyWith<$Res> {
  factory _$$AgentEvent_ToolCallsImplCopyWith(
    _$AgentEvent_ToolCallsImpl value,
    $Res Function(_$AgentEvent_ToolCallsImpl) then,
  ) = __$$AgentEvent_ToolCallsImplCopyWithImpl<$Res>;
  @useResult
  $Res call({List<ToolCallInfo> calls});
}

/// @nodoc
class __$$AgentEvent_ToolCallsImplCopyWithImpl<$Res>
    extends _$AgentEventCopyWithImpl<$Res, _$AgentEvent_ToolCallsImpl>
    implements _$$AgentEvent_ToolCallsImplCopyWith<$Res> {
  __$$AgentEvent_ToolCallsImplCopyWithImpl(
    _$AgentEvent_ToolCallsImpl _value,
    $Res Function(_$AgentEvent_ToolCallsImpl) _then,
  ) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? calls = null}) {
    return _then(
      _$AgentEvent_ToolCallsImpl(
        calls: null == calls
            ? _value._calls
            : calls // ignore: cast_nullable_to_non_nullable
                  as List<ToolCallInfo>,
      ),
    );
  }
}

/// @nodoc

class _$AgentEvent_ToolCallsImpl extends AgentEvent_ToolCalls {
  const _$AgentEvent_ToolCallsImpl({required final List<ToolCallInfo> calls})
    : _calls = calls,
      super._();

  final List<ToolCallInfo> _calls;
  @override
  List<ToolCallInfo> get calls {
    if (_calls is EqualUnmodifiableListView) return _calls;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_calls);
  }

  @override
  String toString() {
    return 'AgentEvent.toolCalls(calls: $calls)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AgentEvent_ToolCallsImpl &&
            const DeepCollectionEquality().equals(other._calls, _calls));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(_calls));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AgentEvent_ToolCallsImplCopyWith<_$AgentEvent_ToolCallsImpl>
  get copyWith =>
      __$$AgentEvent_ToolCallsImplCopyWithImpl<_$AgentEvent_ToolCallsImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String text) content,
    required TResult Function(String text) reasoning,
    required TResult Function(String url) image,
    required TResult Function(List<ToolCallInfo> calls) toolCalls,
    required TResult Function(List<ToolResultInfo> results) toolResults,
    required TResult Function(
      int promptTokens,
      int completionTokens,
      int totalTokens,
    )
    usage,
    required TResult Function() done,
    required TResult Function(String message) error,
  }) {
    return toolCalls(calls);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String text)? content,
    TResult? Function(String text)? reasoning,
    TResult? Function(String url)? image,
    TResult? Function(List<ToolCallInfo> calls)? toolCalls,
    TResult? Function(List<ToolResultInfo> results)? toolResults,
    TResult? Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult? Function()? done,
    TResult? Function(String message)? error,
  }) {
    return toolCalls?.call(calls);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String text)? content,
    TResult Function(String text)? reasoning,
    TResult Function(String url)? image,
    TResult Function(List<ToolCallInfo> calls)? toolCalls,
    TResult Function(List<ToolResultInfo> results)? toolResults,
    TResult Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult Function()? done,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (toolCalls != null) {
      return toolCalls(calls);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AgentEvent_Content value) content,
    required TResult Function(AgentEvent_Reasoning value) reasoning,
    required TResult Function(AgentEvent_Image value) image,
    required TResult Function(AgentEvent_ToolCalls value) toolCalls,
    required TResult Function(AgentEvent_ToolResults value) toolResults,
    required TResult Function(AgentEvent_Usage value) usage,
    required TResult Function(AgentEvent_Done value) done,
    required TResult Function(AgentEvent_Error value) error,
  }) {
    return toolCalls(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AgentEvent_Content value)? content,
    TResult? Function(AgentEvent_Reasoning value)? reasoning,
    TResult? Function(AgentEvent_Image value)? image,
    TResult? Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult? Function(AgentEvent_ToolResults value)? toolResults,
    TResult? Function(AgentEvent_Usage value)? usage,
    TResult? Function(AgentEvent_Done value)? done,
    TResult? Function(AgentEvent_Error value)? error,
  }) {
    return toolCalls?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AgentEvent_Content value)? content,
    TResult Function(AgentEvent_Reasoning value)? reasoning,
    TResult Function(AgentEvent_Image value)? image,
    TResult Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult Function(AgentEvent_ToolResults value)? toolResults,
    TResult Function(AgentEvent_Usage value)? usage,
    TResult Function(AgentEvent_Done value)? done,
    TResult Function(AgentEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (toolCalls != null) {
      return toolCalls(this);
    }
    return orElse();
  }
}

abstract class AgentEvent_ToolCalls extends AgentEvent {
  const factory AgentEvent_ToolCalls({
    required final List<ToolCallInfo> calls,
  }) = _$AgentEvent_ToolCallsImpl;
  const AgentEvent_ToolCalls._() : super._();

  List<ToolCallInfo> get calls;
  @JsonKey(ignore: true)
  _$$AgentEvent_ToolCallsImplCopyWith<_$AgentEvent_ToolCallsImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AgentEvent_ToolResultsImplCopyWith<$Res> {
  factory _$$AgentEvent_ToolResultsImplCopyWith(
    _$AgentEvent_ToolResultsImpl value,
    $Res Function(_$AgentEvent_ToolResultsImpl) then,
  ) = __$$AgentEvent_ToolResultsImplCopyWithImpl<$Res>;
  @useResult
  $Res call({List<ToolResultInfo> results});
}

/// @nodoc
class __$$AgentEvent_ToolResultsImplCopyWithImpl<$Res>
    extends _$AgentEventCopyWithImpl<$Res, _$AgentEvent_ToolResultsImpl>
    implements _$$AgentEvent_ToolResultsImplCopyWith<$Res> {
  __$$AgentEvent_ToolResultsImplCopyWithImpl(
    _$AgentEvent_ToolResultsImpl _value,
    $Res Function(_$AgentEvent_ToolResultsImpl) _then,
  ) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? results = null}) {
    return _then(
      _$AgentEvent_ToolResultsImpl(
        results: null == results
            ? _value._results
            : results // ignore: cast_nullable_to_non_nullable
                  as List<ToolResultInfo>,
      ),
    );
  }
}

/// @nodoc

class _$AgentEvent_ToolResultsImpl extends AgentEvent_ToolResults {
  const _$AgentEvent_ToolResultsImpl({
    required final List<ToolResultInfo> results,
  }) : _results = results,
       super._();

  final List<ToolResultInfo> _results;
  @override
  List<ToolResultInfo> get results {
    if (_results is EqualUnmodifiableListView) return _results;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_results);
  }

  @override
  String toString() {
    return 'AgentEvent.toolResults(results: $results)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AgentEvent_ToolResultsImpl &&
            const DeepCollectionEquality().equals(other._results, _results));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, const DeepCollectionEquality().hash(_results));

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AgentEvent_ToolResultsImplCopyWith<_$AgentEvent_ToolResultsImpl>
  get copyWith =>
      __$$AgentEvent_ToolResultsImplCopyWithImpl<_$AgentEvent_ToolResultsImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String text) content,
    required TResult Function(String text) reasoning,
    required TResult Function(String url) image,
    required TResult Function(List<ToolCallInfo> calls) toolCalls,
    required TResult Function(List<ToolResultInfo> results) toolResults,
    required TResult Function(
      int promptTokens,
      int completionTokens,
      int totalTokens,
    )
    usage,
    required TResult Function() done,
    required TResult Function(String message) error,
  }) {
    return toolResults(results);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String text)? content,
    TResult? Function(String text)? reasoning,
    TResult? Function(String url)? image,
    TResult? Function(List<ToolCallInfo> calls)? toolCalls,
    TResult? Function(List<ToolResultInfo> results)? toolResults,
    TResult? Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult? Function()? done,
    TResult? Function(String message)? error,
  }) {
    return toolResults?.call(results);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String text)? content,
    TResult Function(String text)? reasoning,
    TResult Function(String url)? image,
    TResult Function(List<ToolCallInfo> calls)? toolCalls,
    TResult Function(List<ToolResultInfo> results)? toolResults,
    TResult Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult Function()? done,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (toolResults != null) {
      return toolResults(results);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AgentEvent_Content value) content,
    required TResult Function(AgentEvent_Reasoning value) reasoning,
    required TResult Function(AgentEvent_Image value) image,
    required TResult Function(AgentEvent_ToolCalls value) toolCalls,
    required TResult Function(AgentEvent_ToolResults value) toolResults,
    required TResult Function(AgentEvent_Usage value) usage,
    required TResult Function(AgentEvent_Done value) done,
    required TResult Function(AgentEvent_Error value) error,
  }) {
    return toolResults(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AgentEvent_Content value)? content,
    TResult? Function(AgentEvent_Reasoning value)? reasoning,
    TResult? Function(AgentEvent_Image value)? image,
    TResult? Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult? Function(AgentEvent_ToolResults value)? toolResults,
    TResult? Function(AgentEvent_Usage value)? usage,
    TResult? Function(AgentEvent_Done value)? done,
    TResult? Function(AgentEvent_Error value)? error,
  }) {
    return toolResults?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AgentEvent_Content value)? content,
    TResult Function(AgentEvent_Reasoning value)? reasoning,
    TResult Function(AgentEvent_Image value)? image,
    TResult Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult Function(AgentEvent_ToolResults value)? toolResults,
    TResult Function(AgentEvent_Usage value)? usage,
    TResult Function(AgentEvent_Done value)? done,
    TResult Function(AgentEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (toolResults != null) {
      return toolResults(this);
    }
    return orElse();
  }
}

abstract class AgentEvent_ToolResults extends AgentEvent {
  const factory AgentEvent_ToolResults({
    required final List<ToolResultInfo> results,
  }) = _$AgentEvent_ToolResultsImpl;
  const AgentEvent_ToolResults._() : super._();

  List<ToolResultInfo> get results;
  @JsonKey(ignore: true)
  _$$AgentEvent_ToolResultsImplCopyWith<_$AgentEvent_ToolResultsImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AgentEvent_UsageImplCopyWith<$Res> {
  factory _$$AgentEvent_UsageImplCopyWith(
    _$AgentEvent_UsageImpl value,
    $Res Function(_$AgentEvent_UsageImpl) then,
  ) = __$$AgentEvent_UsageImplCopyWithImpl<$Res>;
  @useResult
  $Res call({int promptTokens, int completionTokens, int totalTokens});
}

/// @nodoc
class __$$AgentEvent_UsageImplCopyWithImpl<$Res>
    extends _$AgentEventCopyWithImpl<$Res, _$AgentEvent_UsageImpl>
    implements _$$AgentEvent_UsageImplCopyWith<$Res> {
  __$$AgentEvent_UsageImplCopyWithImpl(
    _$AgentEvent_UsageImpl _value,
    $Res Function(_$AgentEvent_UsageImpl) _then,
  ) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? promptTokens = null,
    Object? completionTokens = null,
    Object? totalTokens = null,
  }) {
    return _then(
      _$AgentEvent_UsageImpl(
        promptTokens: null == promptTokens
            ? _value.promptTokens
            : promptTokens // ignore: cast_nullable_to_non_nullable
                  as int,
        completionTokens: null == completionTokens
            ? _value.completionTokens
            : completionTokens // ignore: cast_nullable_to_non_nullable
                  as int,
        totalTokens: null == totalTokens
            ? _value.totalTokens
            : totalTokens // ignore: cast_nullable_to_non_nullable
                  as int,
      ),
    );
  }
}

/// @nodoc

class _$AgentEvent_UsageImpl extends AgentEvent_Usage {
  const _$AgentEvent_UsageImpl({
    required this.promptTokens,
    required this.completionTokens,
    required this.totalTokens,
  }) : super._();

  @override
  final int promptTokens;
  @override
  final int completionTokens;
  @override
  final int totalTokens;

  @override
  String toString() {
    return 'AgentEvent.usage(promptTokens: $promptTokens, completionTokens: $completionTokens, totalTokens: $totalTokens)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AgentEvent_UsageImpl &&
            (identical(other.promptTokens, promptTokens) ||
                other.promptTokens == promptTokens) &&
            (identical(other.completionTokens, completionTokens) ||
                other.completionTokens == completionTokens) &&
            (identical(other.totalTokens, totalTokens) ||
                other.totalTokens == totalTokens));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, promptTokens, completionTokens, totalTokens);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AgentEvent_UsageImplCopyWith<_$AgentEvent_UsageImpl> get copyWith =>
      __$$AgentEvent_UsageImplCopyWithImpl<_$AgentEvent_UsageImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String text) content,
    required TResult Function(String text) reasoning,
    required TResult Function(String url) image,
    required TResult Function(List<ToolCallInfo> calls) toolCalls,
    required TResult Function(List<ToolResultInfo> results) toolResults,
    required TResult Function(
      int promptTokens,
      int completionTokens,
      int totalTokens,
    )
    usage,
    required TResult Function() done,
    required TResult Function(String message) error,
  }) {
    return usage(promptTokens, completionTokens, totalTokens);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String text)? content,
    TResult? Function(String text)? reasoning,
    TResult? Function(String url)? image,
    TResult? Function(List<ToolCallInfo> calls)? toolCalls,
    TResult? Function(List<ToolResultInfo> results)? toolResults,
    TResult? Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult? Function()? done,
    TResult? Function(String message)? error,
  }) {
    return usage?.call(promptTokens, completionTokens, totalTokens);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String text)? content,
    TResult Function(String text)? reasoning,
    TResult Function(String url)? image,
    TResult Function(List<ToolCallInfo> calls)? toolCalls,
    TResult Function(List<ToolResultInfo> results)? toolResults,
    TResult Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult Function()? done,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (usage != null) {
      return usage(promptTokens, completionTokens, totalTokens);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AgentEvent_Content value) content,
    required TResult Function(AgentEvent_Reasoning value) reasoning,
    required TResult Function(AgentEvent_Image value) image,
    required TResult Function(AgentEvent_ToolCalls value) toolCalls,
    required TResult Function(AgentEvent_ToolResults value) toolResults,
    required TResult Function(AgentEvent_Usage value) usage,
    required TResult Function(AgentEvent_Done value) done,
    required TResult Function(AgentEvent_Error value) error,
  }) {
    return usage(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AgentEvent_Content value)? content,
    TResult? Function(AgentEvent_Reasoning value)? reasoning,
    TResult? Function(AgentEvent_Image value)? image,
    TResult? Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult? Function(AgentEvent_ToolResults value)? toolResults,
    TResult? Function(AgentEvent_Usage value)? usage,
    TResult? Function(AgentEvent_Done value)? done,
    TResult? Function(AgentEvent_Error value)? error,
  }) {
    return usage?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AgentEvent_Content value)? content,
    TResult Function(AgentEvent_Reasoning value)? reasoning,
    TResult Function(AgentEvent_Image value)? image,
    TResult Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult Function(AgentEvent_ToolResults value)? toolResults,
    TResult Function(AgentEvent_Usage value)? usage,
    TResult Function(AgentEvent_Done value)? done,
    TResult Function(AgentEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (usage != null) {
      return usage(this);
    }
    return orElse();
  }
}

abstract class AgentEvent_Usage extends AgentEvent {
  const factory AgentEvent_Usage({
    required final int promptTokens,
    required final int completionTokens,
    required final int totalTokens,
  }) = _$AgentEvent_UsageImpl;
  const AgentEvent_Usage._() : super._();

  int get promptTokens;
  int get completionTokens;
  int get totalTokens;
  @JsonKey(ignore: true)
  _$$AgentEvent_UsageImplCopyWith<_$AgentEvent_UsageImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$AgentEvent_DoneImplCopyWith<$Res> {
  factory _$$AgentEvent_DoneImplCopyWith(
    _$AgentEvent_DoneImpl value,
    $Res Function(_$AgentEvent_DoneImpl) then,
  ) = __$$AgentEvent_DoneImplCopyWithImpl<$Res>;
}

/// @nodoc
class __$$AgentEvent_DoneImplCopyWithImpl<$Res>
    extends _$AgentEventCopyWithImpl<$Res, _$AgentEvent_DoneImpl>
    implements _$$AgentEvent_DoneImplCopyWith<$Res> {
  __$$AgentEvent_DoneImplCopyWithImpl(
    _$AgentEvent_DoneImpl _value,
    $Res Function(_$AgentEvent_DoneImpl) _then,
  ) : super(_value, _then);
}

/// @nodoc

class _$AgentEvent_DoneImpl extends AgentEvent_Done {
  const _$AgentEvent_DoneImpl() : super._();

  @override
  String toString() {
    return 'AgentEvent.done()';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType && other is _$AgentEvent_DoneImpl);
  }

  @override
  int get hashCode => runtimeType.hashCode;

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String text) content,
    required TResult Function(String text) reasoning,
    required TResult Function(String url) image,
    required TResult Function(List<ToolCallInfo> calls) toolCalls,
    required TResult Function(List<ToolResultInfo> results) toolResults,
    required TResult Function(
      int promptTokens,
      int completionTokens,
      int totalTokens,
    )
    usage,
    required TResult Function() done,
    required TResult Function(String message) error,
  }) {
    return done();
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String text)? content,
    TResult? Function(String text)? reasoning,
    TResult? Function(String url)? image,
    TResult? Function(List<ToolCallInfo> calls)? toolCalls,
    TResult? Function(List<ToolResultInfo> results)? toolResults,
    TResult? Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult? Function()? done,
    TResult? Function(String message)? error,
  }) {
    return done?.call();
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String text)? content,
    TResult Function(String text)? reasoning,
    TResult Function(String url)? image,
    TResult Function(List<ToolCallInfo> calls)? toolCalls,
    TResult Function(List<ToolResultInfo> results)? toolResults,
    TResult Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult Function()? done,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (done != null) {
      return done();
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AgentEvent_Content value) content,
    required TResult Function(AgentEvent_Reasoning value) reasoning,
    required TResult Function(AgentEvent_Image value) image,
    required TResult Function(AgentEvent_ToolCalls value) toolCalls,
    required TResult Function(AgentEvent_ToolResults value) toolResults,
    required TResult Function(AgentEvent_Usage value) usage,
    required TResult Function(AgentEvent_Done value) done,
    required TResult Function(AgentEvent_Error value) error,
  }) {
    return done(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AgentEvent_Content value)? content,
    TResult? Function(AgentEvent_Reasoning value)? reasoning,
    TResult? Function(AgentEvent_Image value)? image,
    TResult? Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult? Function(AgentEvent_ToolResults value)? toolResults,
    TResult? Function(AgentEvent_Usage value)? usage,
    TResult? Function(AgentEvent_Done value)? done,
    TResult? Function(AgentEvent_Error value)? error,
  }) {
    return done?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AgentEvent_Content value)? content,
    TResult Function(AgentEvent_Reasoning value)? reasoning,
    TResult Function(AgentEvent_Image value)? image,
    TResult Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult Function(AgentEvent_ToolResults value)? toolResults,
    TResult Function(AgentEvent_Usage value)? usage,
    TResult Function(AgentEvent_Done value)? done,
    TResult Function(AgentEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (done != null) {
      return done(this);
    }
    return orElse();
  }
}

abstract class AgentEvent_Done extends AgentEvent {
  const factory AgentEvent_Done() = _$AgentEvent_DoneImpl;
  const AgentEvent_Done._() : super._();
}

/// @nodoc
abstract class _$$AgentEvent_ErrorImplCopyWith<$Res> {
  factory _$$AgentEvent_ErrorImplCopyWith(
    _$AgentEvent_ErrorImpl value,
    $Res Function(_$AgentEvent_ErrorImpl) then,
  ) = __$$AgentEvent_ErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$AgentEvent_ErrorImplCopyWithImpl<$Res>
    extends _$AgentEventCopyWithImpl<$Res, _$AgentEvent_ErrorImpl>
    implements _$$AgentEvent_ErrorImplCopyWith<$Res> {
  __$$AgentEvent_ErrorImplCopyWithImpl(
    _$AgentEvent_ErrorImpl _value,
    $Res Function(_$AgentEvent_ErrorImpl) _then,
  ) : super(_value, _then);

  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? message = null}) {
    return _then(
      _$AgentEvent_ErrorImpl(
        message: null == message
            ? _value.message
            : message // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$AgentEvent_ErrorImpl extends AgentEvent_Error {
  const _$AgentEvent_ErrorImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  String toString() {
    return 'AgentEvent.error(message: $message)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$AgentEvent_ErrorImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  @JsonKey(ignore: true)
  @override
  @pragma('vm:prefer-inline')
  _$$AgentEvent_ErrorImplCopyWith<_$AgentEvent_ErrorImpl> get copyWith =>
      __$$AgentEvent_ErrorImplCopyWithImpl<_$AgentEvent_ErrorImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(String text) content,
    required TResult Function(String text) reasoning,
    required TResult Function(String url) image,
    required TResult Function(List<ToolCallInfo> calls) toolCalls,
    required TResult Function(List<ToolResultInfo> results) toolResults,
    required TResult Function(
      int promptTokens,
      int completionTokens,
      int totalTokens,
    )
    usage,
    required TResult Function() done,
    required TResult Function(String message) error,
  }) {
    return error(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(String text)? content,
    TResult? Function(String text)? reasoning,
    TResult? Function(String url)? image,
    TResult? Function(List<ToolCallInfo> calls)? toolCalls,
    TResult? Function(List<ToolResultInfo> results)? toolResults,
    TResult? Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult? Function()? done,
    TResult? Function(String message)? error,
  }) {
    return error?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(String text)? content,
    TResult Function(String text)? reasoning,
    TResult Function(String url)? image,
    TResult Function(List<ToolCallInfo> calls)? toolCalls,
    TResult Function(List<ToolResultInfo> results)? toolResults,
    TResult Function(int promptTokens, int completionTokens, int totalTokens)?
    usage,
    TResult Function()? done,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (error != null) {
      return error(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(AgentEvent_Content value) content,
    required TResult Function(AgentEvent_Reasoning value) reasoning,
    required TResult Function(AgentEvent_Image value) image,
    required TResult Function(AgentEvent_ToolCalls value) toolCalls,
    required TResult Function(AgentEvent_ToolResults value) toolResults,
    required TResult Function(AgentEvent_Usage value) usage,
    required TResult Function(AgentEvent_Done value) done,
    required TResult Function(AgentEvent_Error value) error,
  }) {
    return error(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(AgentEvent_Content value)? content,
    TResult? Function(AgentEvent_Reasoning value)? reasoning,
    TResult? Function(AgentEvent_Image value)? image,
    TResult? Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult? Function(AgentEvent_ToolResults value)? toolResults,
    TResult? Function(AgentEvent_Usage value)? usage,
    TResult? Function(AgentEvent_Done value)? done,
    TResult? Function(AgentEvent_Error value)? error,
  }) {
    return error?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(AgentEvent_Content value)? content,
    TResult Function(AgentEvent_Reasoning value)? reasoning,
    TResult Function(AgentEvent_Image value)? image,
    TResult Function(AgentEvent_ToolCalls value)? toolCalls,
    TResult Function(AgentEvent_ToolResults value)? toolResults,
    TResult Function(AgentEvent_Usage value)? usage,
    TResult Function(AgentEvent_Done value)? done,
    TResult Function(AgentEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (error != null) {
      return error(this);
    }
    return orElse();
  }
}

abstract class AgentEvent_Error extends AgentEvent {
  const factory AgentEvent_Error({required final String message}) =
      _$AgentEvent_ErrorImpl;
  const AgentEvent_Error._() : super._();

  String get message;
  @JsonKey(ignore: true)
  _$$AgentEvent_ErrorImplCopyWith<_$AgentEvent_ErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
